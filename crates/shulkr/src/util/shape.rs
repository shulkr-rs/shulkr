use crate::{
    entity::EntityDimensions,
    util::{Axis, Direction, Point},
};

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct VoxelShape {
    boxes: &'static [&'static Aabb],
}

impl VoxelShape {
    pub const EMPTY: VoxelShape = VoxelShape::new(&[]);
    pub const FULL: VoxelShape = VoxelShape::new(&[&Aabb::FULL]);

    pub const fn new(boxes: &'static [&'static Aabb]) -> Self {
        Self { boxes }
    }

    pub fn intersects(&self, entity: &EntityDimensions, x: f32, y: f32, z: f32) -> bool {
        let entity_aabb = entity.aabb(x, y, z);

        self.boxes.iter().any(|aabb| entity_aabb.intersects(aabb))
    }

    pub const fn boxes(&self) -> &'static [&'static Aabb] {
        self.boxes
    }

    pub fn is_empty(&self) -> bool {
        self.boxes.iter().all(|aabb| aabb.is_empty())
    }

    pub fn min(&self, axis: Axis) -> f64 {
        self.boxes
            .iter()
            .filter(|aabb| !aabb.is_empty())
            .map(|aabb| aabb.min(axis))
            .fold(f64::INFINITY, f64::min)
    }

    pub fn max(&self, axis: Axis) -> f64 {
        self.boxes
            .iter()
            .filter(|aabb| !aabb.is_empty())
            .map(|aabb| aabb.max(axis))
            .fold(f64::NEG_INFINITY, f64::max)
    }

    pub const fn len(&self) -> usize {
        self.boxes.len()
    }

    pub fn iter(&self) -> impl Iterator<Item = &'static Aabb> {
        self.boxes.iter().copied()
    }

    pub fn bounds(&self) -> Option<Aabb> {
        let mut bounds: Option<Aabb> = None;

        for aabb in self.boxes.iter().filter(|aabb| !aabb.is_empty()) {
            bounds = Some(match bounds {
                None => **aabb,
                Some(bounds) => Aabb::new(
                    bounds.min_x.min(aabb.min_x),
                    bounds.min_y.min(aabb.min_y),
                    bounds.min_z.min(aabb.min_z),
                    bounds.max_x.max(aabb.max_x),
                    bounds.max_y.max(aabb.max_y),
                    bounds.max_z.max(aabb.max_z),
                ),
            });
        }

        bounds
    }
}

impl IntoIterator for VoxelShape {
    type Item = &'static Aabb;
    type IntoIter = std::iter::Copied<std::slice::Iter<'static, &'static Aabb>>;

    fn into_iter(self) -> Self::IntoIter {
        self.boxes.iter().copied()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Aabb {
    min_x: f64,
    min_y: f64,
    min_z: f64,
    max_x: f64,
    max_y: f64,
    max_z: f64,
}

impl Aabb {
    pub const EMPTY: Self = Self::new(0.0, 0.0, 0.0, 0.0, 0.0, 0.0);
    pub const FULL: Self = Self::new(0.0, 0.0, 0.0, 1.0, 1.0, 1.0);

    pub const fn new(
        min_x: f64,
        min_y: f64,
        min_z: f64,
        max_x: f64,
        max_y: f64,
        max_z: f64,
    ) -> Self {
        let (min_x, max_x) = ordered(min_x, max_x);
        let (min_y, max_y) = ordered(min_y, max_y);
        let (min_z, max_z) = ordered(min_z, max_z);

        Self {
            min_x,
            min_y,
            min_z,
            max_x,
            max_y,
            max_z,
        }
    }

    pub const fn min(&self, axis: Axis) -> f64 {
        match axis {
            Axis::X => self.min_x,
            Axis::Y => self.min_y,
            Axis::Z => self.min_z,
        }
    }

    pub const fn max(&self, axis: Axis) -> f64 {
        match axis {
            Axis::X => self.max_x,
            Axis::Y => self.max_y,
            Axis::Z => self.max_z,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.min_x >= self.max_x || self.min_y >= self.max_y || self.min_z >= self.max_z
    }

    pub fn translate(&self, delta: Point) -> Self {
        Self::new(
            self.min_x + delta.x(),
            self.min_y + delta.y(),
            self.min_z + delta.z(),
            self.max_x + delta.x(),
            self.max_y + delta.y(),
            self.max_z + delta.z(),
        )
    }

    pub const fn min_x(&self) -> f64 {
        self.min_x
    }

    pub const fn min_y(&self) -> f64 {
        self.min_y
    }

    pub const fn min_z(&self) -> f64 {
        self.min_z
    }

    pub const fn max_x(&self) -> f64 {
        self.max_x
    }

    pub const fn max_y(&self) -> f64 {
        self.max_y
    }

    pub const fn max_z(&self) -> f64 {
        self.max_z
    }

    pub const fn inflate(&self, x: f64, y: f64, z: f64) -> Self {
        Self::new(
            self.min_x - x,
            self.min_y - y,
            self.min_z - z,
            self.max_x + x,
            self.max_y + y,
            self.max_z + z,
        )
    }

    pub const fn deflate(&self, amount: f64) -> Self {
        self.inflate(-amount, -amount, -amount)
    }

    pub const fn intersects(&self, other: &Aabb) -> bool {
        self.min_x < other.max_x
            && self.max_x > other.min_x
            && self.min_y < other.max_y
            && self.max_y > other.min_y
            && self.min_z < other.max_z
            && self.max_z > other.min_z
    }
}

const fn ordered(a: f64, b: f64) -> (f64, f64) {
    if a <= b { (a, b) } else { (b, a) }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BooleanOp {
    False,
    NotOr,
    OnlySecond,
    NotFirst,
    OnlyFirst,
    NotSecond,
    NotSame,
    NotAnd,
    And,
    Same,
    Second,
    Causes,
    First,
    CausedBy,
    Or,
    True,
}

impl BooleanOp {
    pub const fn apply(self, first: bool, second: bool) -> bool {
        match self {
            Self::False => false,
            Self::NotOr => !first && !second,
            Self::OnlySecond => second && !first,
            Self::NotFirst => !first,
            Self::OnlyFirst => first && !second,
            Self::NotSecond => !second,
            Self::NotSame => first != second,
            Self::NotAnd => !first || !second,
            Self::And => first && second,
            Self::Same => first == second,
            Self::Second => second,
            Self::Causes => !first || second,
            Self::First => first,
            Self::CausedBy => first || !second,
            Self::Or => first || second,
            Self::True => true,
        }
    }
}
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct OffsetVoxelShape {
    shape: VoxelShape,
    offset: Point,
}

impl OffsetVoxelShape {
    pub const fn new(shape: VoxelShape, offset: Point) -> Self {
        Self { shape, offset }
    }

    pub const fn without_offset(shape: VoxelShape) -> Self {
        Self {
            shape,
            offset: Point::ZERO,
        }
    }

    pub const fn shape(self) -> VoxelShape {
        self.shape
    }

    pub const fn offset(self) -> Point {
        self.offset
    }

    pub fn is_empty(self) -> bool {
        self.shape.is_empty()
    }

    pub fn iter(self) -> impl Iterator<Item = Aabb> {
        self.shape
            .into_iter()
            .map(move |aabb| aabb.translate(self.offset))
    }

    pub fn min(self, axis: Axis) -> f64 {
        self.shape.min(axis) + axis_offset(self.offset, axis)
    }

    pub fn max(self, axis: Axis) -> f64 {
        self.shape.max(axis) + axis_offset(self.offset, axis)
    }

    pub fn bounds(self) -> Option<Aabb> {
        self.shape
            .bounds()
            .map(|bounds| bounds.translate(self.offset))
    }
}

const fn axis_offset(offset: Point, axis: Axis) -> f64 {
    match axis {
        Axis::X => offset.x(),
        Axis::Y => offset.y(),
        Axis::Z => offset.z(),
    }
}

const VOXEL_EPSILON: f64 = 1.0e-7;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ShapeKind {
    Collision,
    Support,
    Outline,
    Occlusion,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ShapeOffsetFlags {
    collision: bool,
    support: bool,
    outline: bool,
    occlusion: bool,
}

impl ShapeOffsetFlags {
    pub const NONE: Self = Self::new(false, false, false, false);

    pub const fn new(collision: bool, support: bool, outline: bool, occlusion: bool) -> Self {
        Self {
            collision,
            support,
            outline,
            occlusion,
        }
    }

    pub const fn uses_offset(self, kind: ShapeKind) -> bool {
        match kind {
            ShapeKind::Collision => self.collision,
            ShapeKind::Support => self.support,
            ShapeKind::Outline => self.outline,
            ShapeKind::Occlusion => self.occlusion,
        }
    }
}

pub fn bounding_box(shape: VoxelShape) -> Aabb {
    match shape.bounds() {
        Some(bounds) => bounds,
        None => panic!("bounding_box called on empty shape"),
    }
}

pub fn is_shape_full_block(shape: VoxelShape) -> bool {
    !join_is_not_empty(VoxelShape::FULL, shape, BooleanOp::NotSame)
}

pub fn is_offset_shape_full_block(shape: OffsetVoxelShape) -> bool {
    if shape.offset == Point::ZERO {
        return is_shape_full_block(shape.shape);
    }

    if shape.is_empty()
        || shape.min(Axis::X) > VOXEL_EPSILON
        || shape.max(Axis::X) < 1.0 - VOXEL_EPSILON
        || shape.min(Axis::Y) > VOXEL_EPSILON
        || shape.max(Axis::Y) < 1.0 - VOXEL_EPSILON
        || shape.min(Axis::Z) > VOXEL_EPSILON
        || shape.max(Axis::Z) < 1.0 - VOXEL_EPSILON
    {
        return false;
    }

    let mut x_edges = vec![0.0, 1.0];
    let mut y_edges = vec![0.0, 1.0];
    let mut z_edges = vec![0.0, 1.0];
    for aabb in shape.iter() {
        if aabb.is_empty() {
            continue;
        }
        if aabb.max_x() > VOXEL_EPSILON && aabb.min_x() < 1.0 - VOXEL_EPSILON {
            x_edges.push(aabb.min_x().clamp(0.0, 1.0));
            x_edges.push(aabb.max_x().clamp(0.0, 1.0));
        }
        if aabb.max_y() > VOXEL_EPSILON && aabb.min_y() < 1.0 - VOXEL_EPSILON {
            y_edges.push(aabb.min_y().clamp(0.0, 1.0));
            y_edges.push(aabb.max_y().clamp(0.0, 1.0));
        }
        if aabb.max_z() > VOXEL_EPSILON && aabb.min_z() < 1.0 - VOXEL_EPSILON {
            z_edges.push(aabb.min_z().clamp(0.0, 1.0));
            z_edges.push(aabb.max_z().clamp(0.0, 1.0));
        }
    }
    sort_and_dedup_voxel_edges(&mut x_edges);
    sort_and_dedup_voxel_edges(&mut y_edges);
    sort_and_dedup_voxel_edges(&mut z_edges);

    for x in x_edges.windows(2) {
        if x[1] - x[0] <= VOXEL_EPSILON {
            continue;
        }
        for y in y_edges.windows(2) {
            if y[1] - y[0] <= VOXEL_EPSILON {
                continue;
            }
            for z in z_edges.windows(2) {
                if z[1] - z[0] <= VOXEL_EPSILON {
                    continue;
                }
                if !offset_shape_fills_cell(shape, x[0], x[1], y[0], y[1], z[0], z[1]) {
                    return false;
                }
            }
        }
    }

    true
}

pub fn join_is_not_empty(first: VoxelShape, second: VoxelShape, op: BooleanOp) -> bool {
    assert!(
        !op.apply(false, false),
        "join_is_not_empty cannot use an operation that includes empty outside space"
    );

    let first_empty = first.is_empty();
    let second_empty = second.is_empty();
    if first_empty || second_empty {
        return op.apply(!first_empty, !second_empty);
    }

    if first == second {
        return op.apply(true, true);
    }

    let first_only_matters = op.apply(true, false);
    let second_only_matters = op.apply(false, true);
    for axis in [Axis::X, Axis::Y, Axis::Z] {
        if first.max(axis) < second.min(axis) - VOXEL_EPSILON {
            return first_only_matters || second_only_matters;
        }
        if second.max(axis) < first.min(axis) - VOXEL_EPSILON {
            return first_only_matters || second_only_matters;
        }
    }

    let mut x_edges = shape_edges(first, second, Axis::X);
    let mut y_edges = shape_edges(first, second, Axis::Y);
    let mut z_edges = shape_edges(first, second, Axis::Z);
    sort_and_dedup_voxel_edges(&mut x_edges);
    sort_and_dedup_voxel_edges(&mut y_edges);
    sort_and_dedup_voxel_edges(&mut z_edges);

    for x in x_edges.windows(2) {
        if x[1] - x[0] <= VOXEL_EPSILON {
            continue;
        }
        for y in y_edges.windows(2) {
            if y[1] - y[0] <= VOXEL_EPSILON {
                continue;
            }
            for z in z_edges.windows(2) {
                if z[1] - z[0] <= VOXEL_EPSILON {
                    continue;
                }
                let first_full = shape_fills_cell(first, x[0], x[1], y[0], y[1], z[0], z[1]);
                let second_full = shape_fills_cell(second, x[0], x[1], y[0], y[1], z[0], z[1]);
                if op.apply(first_full, second_full) {
                    return true;
                }
            }
        }
    }

    false
}

pub fn join_unoptimized_boxes(first: VoxelShape, second: VoxelShape, op: BooleanOp) -> Vec<Aabb> {
    assert!(
        !op.apply(false, false),
        "join_unoptimized_boxes cannot use an operation that includes empty outside space"
    );

    if first.is_empty() && second.is_empty() {
        return Vec::new();
    }

    let mut x_edges = shape_edges(first, second, Axis::X);
    let mut y_edges = shape_edges(first, second, Axis::Y);
    let mut z_edges = shape_edges(first, second, Axis::Z);
    sort_and_dedup_voxel_edges(&mut x_edges);
    sort_and_dedup_voxel_edges(&mut y_edges);
    sort_and_dedup_voxel_edges(&mut z_edges);

    let mut boxes = Vec::new();
    for x in x_edges.windows(2) {
        if x[1] - x[0] <= VOXEL_EPSILON {
            continue;
        }
        for y in y_edges.windows(2) {
            if y[1] - y[0] <= VOXEL_EPSILON {
                continue;
            }
            for z in z_edges.windows(2) {
                if z[1] - z[0] <= VOXEL_EPSILON {
                    continue;
                }

                let first_full = shape_fills_cell(first, x[0], x[1], y[0], y[1], z[0], z[1]);
                let second_full = shape_fills_cell(second, x[0], x[1], y[0], y[1], z[0], z[1]);
                if op.apply(first_full, second_full) {
                    boxes.push(Aabb::new(x[0], y[0], z[0], x[1], y[1], z[1]));
                }
            }
        }
    }

    boxes
}

fn shape_edges(first: VoxelShape, second: VoxelShape, axis: Axis) -> Vec<f64> {
    let mut edges = Vec::with_capacity((first.len() + second.len()) * 2);
    for shape in [first, second] {
        for aabb in shape {
            if aabb.is_empty() {
                continue;
            }
            edges.push(aabb.min(axis));
            edges.push(aabb.max(axis));
        }
    }
    edges
}

fn sort_and_dedup_voxel_edges(edges: &mut Vec<f64>) {
    edges.sort_by(f64::total_cmp);
    edges.dedup_by(|a, b| (*a - *b).abs() <= VOXEL_EPSILON);
}

fn shape_fills_cell(
    shape: VoxelShape,
    min_x: f64,
    max_x: f64,
    min_y: f64,
    max_y: f64,
    min_z: f64,
    max_z: f64,
) -> bool {
    shape.into_iter().any(|aabb| {
        !aabb.is_empty()
            && aabb.min_x() <= min_x + VOXEL_EPSILON
            && aabb.max_x() >= max_x - VOXEL_EPSILON
            && aabb.min_y() <= min_y + VOXEL_EPSILON
            && aabb.max_y() >= max_y - VOXEL_EPSILON
            && aabb.min_z() <= min_z + VOXEL_EPSILON
            && aabb.max_z() >= max_z - VOXEL_EPSILON
    })
}

fn offset_shape_fills_cell(
    shape: OffsetVoxelShape,
    min_x: f64,
    max_x: f64,
    min_y: f64,
    max_y: f64,
    min_z: f64,
    max_z: f64,
) -> bool {
    shape.iter().any(|aabb| {
        !aabb.is_empty()
            && aabb.min_x() <= min_x + VOXEL_EPSILON
            && aabb.max_x() >= max_x - VOXEL_EPSILON
            && aabb.min_y() <= min_y + VOXEL_EPSILON
            && aabb.max_y() >= max_y - VOXEL_EPSILON
            && aabb.min_z() <= min_z + VOXEL_EPSILON
            && aabb.max_z() >= max_z - VOXEL_EPSILON
    })
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SupportType {
    Full,
    Center,
    Rigid,
}

const CENTER_SUPPORT_MIN: f64 = 7.0 / 16.0;
const CENTER_SUPPORT_MAX: f64 = 9.0 / 16.0;
const CENTER_SUPPORT_Y_MAX: f64 = 10.0 / 16.0;

const RIGID_BORDER: f64 = 0.125;

pub fn is_face_full(shape: VoxelShape, direction: Direction) -> bool {
    face_rectangles_cover(shape, direction, 0.0, 1.0, 0.0, 1.0)
}

pub fn is_offset_face_full(shape: OffsetVoxelShape, direction: Direction) -> bool {
    offset_face_rectangles_cover(shape, direction, 0.0, 1.0, 0.0, 1.0)
}

pub fn is_face_center_supported(shape: VoxelShape, direction: Direction) -> bool {
    if shape.is_empty() {
        return false;
    }

    match direction {
        Direction::Down | Direction::Up => face_rectangles_cover(
            shape,
            direction,
            CENTER_SUPPORT_MIN,
            CENTER_SUPPORT_MAX,
            CENTER_SUPPORT_MIN,
            CENTER_SUPPORT_MAX,
        ),
        Direction::North | Direction::South => face_rectangles_cover(
            shape,
            direction,
            CENTER_SUPPORT_MIN,
            CENTER_SUPPORT_MAX,
            0.0,
            CENTER_SUPPORT_Y_MAX,
        ),
        Direction::West | Direction::East => face_rectangles_cover(
            shape,
            direction,
            0.0,
            CENTER_SUPPORT_Y_MAX,
            CENTER_SUPPORT_MIN,
            CENTER_SUPPORT_MAX,
        ),
    }
}

pub fn is_offset_face_center_supported(shape: OffsetVoxelShape, direction: Direction) -> bool {
    if shape.is_empty() {
        return false;
    }

    match direction {
        Direction::Down | Direction::Up => offset_face_rectangles_cover(
            shape,
            direction,
            CENTER_SUPPORT_MIN,
            CENTER_SUPPORT_MAX,
            CENTER_SUPPORT_MIN,
            CENTER_SUPPORT_MAX,
        ),
        Direction::North | Direction::South => offset_face_rectangles_cover(
            shape,
            direction,
            CENTER_SUPPORT_MIN,
            CENTER_SUPPORT_MAX,
            0.0,
            CENTER_SUPPORT_Y_MAX,
        ),
        Direction::West | Direction::East => offset_face_rectangles_cover(
            shape,
            direction,
            0.0,
            CENTER_SUPPORT_Y_MAX,
            CENTER_SUPPORT_MIN,
            CENTER_SUPPORT_MAX,
        ),
    }
}

pub fn is_face_rigid_supported(shape: VoxelShape, direction: Direction) -> bool {
    if shape.is_empty() {
        return false;
    }

    match direction {
        Direction::Down | Direction::Up => {
            face_rectangles_cover(shape, direction, 0.0, RIGID_BORDER, 0.0, 1.0)
                && face_rectangles_cover(shape, direction, 1.0 - RIGID_BORDER, 1.0, 0.0, 1.0)
                && face_rectangles_cover(
                    shape,
                    direction,
                    RIGID_BORDER,
                    1.0 - RIGID_BORDER,
                    0.0,
                    RIGID_BORDER,
                )
                && face_rectangles_cover(
                    shape,
                    direction,
                    RIGID_BORDER,
                    1.0 - RIGID_BORDER,
                    1.0 - RIGID_BORDER,
                    1.0,
                )
        }
        Direction::North | Direction::South | Direction::West | Direction::East => {
            is_face_full(shape, direction)
        }
    }
}

pub fn is_offset_face_rigid_supported(shape: OffsetVoxelShape, direction: Direction) -> bool {
    if shape.is_empty() {
        return false;
    }

    match direction {
        Direction::Down | Direction::Up => {
            offset_face_rectangles_cover(shape, direction, 0.0, RIGID_BORDER, 0.0, 1.0)
                && offset_face_rectangles_cover(shape, direction, 1.0 - RIGID_BORDER, 1.0, 0.0, 1.0)
                && offset_face_rectangles_cover(
                    shape,
                    direction,
                    RIGID_BORDER,
                    1.0 - RIGID_BORDER,
                    0.0,
                    RIGID_BORDER,
                )
                && offset_face_rectangles_cover(
                    shape,
                    direction,
                    RIGID_BORDER,
                    1.0 - RIGID_BORDER,
                    1.0 - RIGID_BORDER,
                    1.0,
                )
        }
        Direction::North | Direction::South | Direction::West | Direction::East => {
            is_offset_face_full(shape, direction)
        }
    }
}

pub fn is_side_solid(shape: VoxelShape, direction: Direction, support_type: SupportType) -> bool {
    match support_type {
        SupportType::Full => is_face_full(shape, direction),
        SupportType::Center => is_face_center_supported(shape, direction),
        SupportType::Rigid => is_face_rigid_supported(shape, direction),
    }
}

pub fn is_offset_face_sturdy(
    shape: OffsetVoxelShape,
    direction: Direction,
    support_type: SupportType,
) -> bool {
    match support_type {
        SupportType::Full => is_offset_face_full(shape, direction),
        SupportType::Center => is_offset_face_center_supported(shape, direction),
        SupportType::Rigid => is_offset_face_rigid_supported(shape, direction),
    }
}

pub fn is_block_local_face_sturdy(
    boxes: &[&Aabb],
    direction: Direction,
    support_type: SupportType,
) -> bool {
    match support_type {
        SupportType::Full => {
            block_local_face_rectangles_cover(boxes, direction, 0.0, 1.0, 0.0, 1.0)
        }
        SupportType::Center => match direction {
            Direction::Down | Direction::Up => block_local_face_rectangles_cover(
                boxes,
                direction,
                CENTER_SUPPORT_MIN,
                CENTER_SUPPORT_MAX,
                CENTER_SUPPORT_MIN,
                CENTER_SUPPORT_MAX,
            ),
            Direction::North | Direction::South => block_local_face_rectangles_cover(
                boxes,
                direction,
                CENTER_SUPPORT_MIN,
                CENTER_SUPPORT_MAX,
                0.0,
                CENTER_SUPPORT_Y_MAX,
            ),
            Direction::West | Direction::East => block_local_face_rectangles_cover(
                boxes,
                direction,
                0.0,
                CENTER_SUPPORT_Y_MAX,
                CENTER_SUPPORT_MIN,
                CENTER_SUPPORT_MAX,
            ),
        },
        SupportType::Rigid => match direction {
            Direction::Down | Direction::Up => {
                block_local_face_rectangles_cover(boxes, direction, 0.0, RIGID_BORDER, 0.0, 1.0)
                    && block_local_face_rectangles_cover(
                        boxes,
                        direction,
                        1.0 - RIGID_BORDER,
                        1.0,
                        0.0,
                        1.0,
                    )
                    && block_local_face_rectangles_cover(
                        boxes,
                        direction,
                        RIGID_BORDER,
                        1.0 - RIGID_BORDER,
                        0.0,
                        RIGID_BORDER,
                    )
                    && block_local_face_rectangles_cover(
                        boxes,
                        direction,
                        RIGID_BORDER,
                        1.0 - RIGID_BORDER,
                        1.0 - RIGID_BORDER,
                        1.0,
                    )
            }
            Direction::North | Direction::South | Direction::West | Direction::East => {
                block_local_face_rectangles_cover(boxes, direction, 0.0, 1.0, 0.0, 1.0)
            }
        },
    }
}

#[derive(Clone, Copy)]
struct FaceRect {
    min_a: f64,
    max_a: f64,
    min_b: f64,
    max_b: f64,
}

const FACE_EPSILON: f64 = 1.0e-6;

pub fn face_rectangles_cover(
    shape: VoxelShape,
    direction: Direction,
    target_min_a: f64,
    target_max_a: f64,
    target_min_b: f64,
    target_max_b: f64,
) -> bool {
    block_local_face_rectangles_cover(
        shape.boxes(),
        direction,
        target_min_a,
        target_max_a,
        target_min_b,
        target_max_b,
    )
}

fn block_local_face_rectangles_cover(
    boxes: &[&Aabb],
    direction: Direction,
    target_min_a: f64,
    target_max_a: f64,
    target_min_b: f64,
    target_max_b: f64,
) -> bool {
    let mut rects = Vec::new();
    for aabb in boxes {
        let Some(rect) = face_rect_for_aabb(**aabb, direction) else {
            continue;
        };
        if rect.max_a <= target_min_a
            || rect.min_a >= target_max_a
            || rect.max_b <= target_min_b
            || rect.min_b >= target_max_b
        {
            continue;
        }
        rects.push(FaceRect {
            min_a: rect.min_a.max(target_min_a),
            max_a: rect.max_a.min(target_max_a),
            min_b: rect.min_b.max(target_min_b),
            max_b: rect.max_b.min(target_max_b),
        });
    }

    face_rects_cover_target(
        rects,
        target_min_a,
        target_max_a,
        target_min_b,
        target_max_b,
    )
}

pub fn offset_face_rectangles_cover(
    shape: OffsetVoxelShape,
    direction: Direction,
    target_min_a: f64,
    target_max_a: f64,
    target_min_b: f64,
    target_max_b: f64,
) -> bool {
    let mut rects = Vec::new();
    for aabb in shape.iter() {
        let Some(rect) = face_rect_for_aabb(aabb, direction) else {
            continue;
        };
        if rect.max_a <= target_min_a
            || rect.min_a >= target_max_a
            || rect.max_b <= target_min_b
            || rect.min_b >= target_max_b
        {
            continue;
        }
        rects.push(FaceRect {
            min_a: rect.min_a.max(target_min_a),
            max_a: rect.max_a.min(target_max_a),
            min_b: rect.min_b.max(target_min_b),
            max_b: rect.max_b.min(target_max_b),
        });
    }

    face_rects_cover_target(
        rects,
        target_min_a,
        target_max_a,
        target_min_b,
        target_max_b,
    )
}

fn face_rects_cover_target(
    rects: Vec<FaceRect>,
    target_min_a: f64,
    target_max_a: f64,
    target_min_b: f64,
    target_max_b: f64,
) -> bool {
    if rects.is_empty() {
        return false;
    }

    let mut a_edges = vec![target_min_a, target_max_a];
    let mut b_edges = vec![target_min_b, target_max_b];
    for rect in &rects {
        a_edges.push(rect.min_a);
        a_edges.push(rect.max_a);
        b_edges.push(rect.min_b);
        b_edges.push(rect.max_b);
    }
    sort_and_dedup_edges(&mut a_edges);
    sort_and_dedup_edges(&mut b_edges);

    for a_pair in a_edges.windows(2) {
        if a_pair[1] - a_pair[0] <= FACE_EPSILON {
            continue;
        }
        for b_pair in b_edges.windows(2) {
            if b_pair[1] - b_pair[0] <= FACE_EPSILON {
                continue;
            }
            let covered = rects.iter().any(|rect| {
                rect.min_a <= a_pair[0] + FACE_EPSILON
                    && rect.max_a >= a_pair[1] - FACE_EPSILON
                    && rect.min_b <= b_pair[0] + FACE_EPSILON
                    && rect.max_b >= b_pair[1] - FACE_EPSILON
            });
            if !covered {
                return false;
            }
        }
    }

    true
}

fn face_rect_for_aabb(aabb: Aabb, direction: Direction) -> Option<FaceRect> {
    let rect = match direction {
        Direction::Down if aabb.min_y() <= FACE_EPSILON && aabb.max_y() >= -FACE_EPSILON => {
            FaceRect {
                min_a: aabb.min_x(),
                max_a: aabb.max_x(),
                min_b: aabb.min_z(),
                max_b: aabb.max_z(),
            }
        }
        Direction::Up
            if aabb.max_y() >= 1.0 - FACE_EPSILON && aabb.min_y() <= 1.0 + FACE_EPSILON =>
        {
            FaceRect {
                min_a: aabb.min_x(),
                max_a: aabb.max_x(),
                min_b: aabb.min_z(),
                max_b: aabb.max_z(),
            }
        }
        Direction::North if aabb.min_z() <= FACE_EPSILON && aabb.max_z() >= -FACE_EPSILON => {
            FaceRect {
                min_a: aabb.min_x(),
                max_a: aabb.max_x(),
                min_b: aabb.min_y(),
                max_b: aabb.max_y(),
            }
        }
        Direction::South
            if aabb.max_z() >= 1.0 - FACE_EPSILON && aabb.min_z() <= 1.0 + FACE_EPSILON =>
        {
            FaceRect {
                min_a: aabb.min_x(),
                max_a: aabb.max_x(),
                min_b: aabb.min_y(),
                max_b: aabb.max_y(),
            }
        }
        Direction::West if aabb.min_x() <= FACE_EPSILON && aabb.max_x() >= -FACE_EPSILON => {
            FaceRect {
                min_a: aabb.min_y(),
                max_a: aabb.max_y(),
                min_b: aabb.min_z(),
                max_b: aabb.max_z(),
            }
        }
        Direction::East
            if aabb.max_x() >= 1.0 - FACE_EPSILON && aabb.min_x() <= 1.0 + FACE_EPSILON =>
        {
            FaceRect {
                min_a: aabb.min_y(),
                max_a: aabb.max_y(),
                min_b: aabb.min_z(),
                max_b: aabb.max_z(),
            }
        }
        _ => return None,
    };

    if rect.min_a >= rect.max_a || rect.min_b >= rect.max_b {
        return None;
    }
    Some(rect)
}

fn sort_and_dedup_edges(edges: &mut Vec<f64>) {
    edges.sort_by(f64::total_cmp);
    edges.dedup_by(|a, b| (*a - *b).abs() <= FACE_EPSILON);
}

#[cfg(test)]
mod tests {
    use super::*;

    const QUADRANT_TOP_FACE: &[&Aabb] = &[
        &Aabb::new(0.0, 0.5, 0.0, 0.5, 1.0, 0.5),
        &Aabb::new(0.5, 0.5, 0.0, 1.0, 1.0, 0.5),
        &Aabb::new(0.0, 0.5, 0.5, 0.5, 1.0, 1.0),
        &Aabb::new(0.5, 0.5, 0.5, 1.0, 1.0, 1.0),
    ];

    const GAPPED_TOP_FACE: &[&Aabb] = &[
        &Aabb::new(0.0, 0.5, 0.0, 0.45, 1.0, 1.0),
        &Aabb::new(0.55, 0.5, 0.0, 1.0, 1.0, 1.0),
    ];

    const VANILLA_AZALEA_SHAPE: &[&Aabb] = &[
        &Aabb::new(0.375, 0.0, 0.375, 0.625, 1.0, 0.625),
        &Aabb::new(0.0, 0.5, 0.0, 0.375, 1.0, 1.0),
        &Aabb::new(0.375, 0.5, 0.0, 1.0, 1.0, 0.375),
        &Aabb::new(0.375, 0.5, 0.625, 1.0, 1.0, 1.0),
        &Aabb::new(0.625, 0.5, 0.375, 1.0, 1.0, 0.625),
    ];

    const SPLIT_FULL_BLOCK: &[&Aabb] = &[
        &Aabb::new(0.0, 0.0, 0.0, 0.5, 1.0, 1.0),
        &Aabb::new(0.5, 0.0, 0.0, 1.0, 1.0, 1.0),
    ];

    const Z_GAPPED_BLOCK_WITH_OFFSET: &[&Aabb] = &[
        &Aabb::new(0.0, 0.0, -0.1, 1.0, 1.0, 0.15),
        &Aabb::new(0.0, 0.0, 0.65, 1.0, 1.0, 0.9),
    ];

    const LOWER_HALF_BLOCK: &[&Aabb] = &[&Aabb::new(0.0, 0.0, 0.0, 1.0, 0.5, 1.0)];

    const UPPER_HALF_BLOCK: &[&Aabb] = &[&Aabb::new(0.0, 0.5, 0.0, 1.0, 1.0, 1.0)];

    const OVERLAPPING_HALF_BLOCKS: &[&Aabb] = &[
        &Aabb::new(0.0, 0.0, 0.0, 0.75, 1.0, 1.0),
        &Aabb::new(0.25, 0.0, 0.0, 1.0, 1.0, 1.0),
    ];

    const ZERO_VOLUME_BOX: &[&Aabb] = &[&Aabb::new(0.0, 0.0, 0.0, 1.0, 0.0, 1.0)];

    const RIGID_TOP_RING: &[&Aabb] = &[
        &Aabb::new(0.0, 0.0, 0.0, RIGID_BORDER, 1.0, 1.0),
        &Aabb::new(1.0 - RIGID_BORDER, 0.0, 0.0, 1.0, 1.0, 1.0),
        &Aabb::new(
            RIGID_BORDER,
            0.0,
            0.0,
            1.0 - RIGID_BORDER,
            1.0,
            RIGID_BORDER,
        ),
        &Aabb::new(
            RIGID_BORDER,
            0.0,
            1.0 - RIGID_BORDER,
            1.0 - RIGID_BORDER,
            1.0,
            1.0,
        ),
    ];

    const RIGID_CENTER_PANEL: &[&Aabb] = &[&Aabb::new(
        RIGID_BORDER,
        0.0,
        RIGID_BORDER,
        1.0 - RIGID_BORDER,
        1.0,
        1.0 - RIGID_BORDER,
    )];

    const RIGID_WEST_FACE_RING: &[&Aabb] = &[
        &Aabb::new(0.0, 0.0, 0.0, 1.0, RIGID_BORDER, 1.0),
        &Aabb::new(0.0, 1.0 - RIGID_BORDER, 0.0, 1.0, 1.0, 1.0),
        &Aabb::new(
            0.0,
            RIGID_BORDER,
            0.0,
            1.0,
            1.0 - RIGID_BORDER,
            RIGID_BORDER,
        ),
        &Aabb::new(
            0.0,
            RIGID_BORDER,
            1.0 - RIGID_BORDER,
            1.0,
            1.0 - RIGID_BORDER,
            1.0,
        ),
    ];

    #[test]
    fn boolean_op_matches_vanilla_truth_table() {
        assert!(BooleanOp::OnlyFirst.apply(true, false));
        assert!(!BooleanOp::OnlyFirst.apply(false, true));
        assert!(BooleanOp::NotSame.apply(true, false));
        assert!(!BooleanOp::NotSame.apply(true, true));
        assert!(BooleanOp::Or.apply(false, true));
        assert!(!BooleanOp::And.apply(true, false));
    }

    #[test]
    fn join_is_not_empty_detects_intersection() {
        assert!(join_is_not_empty(
            VoxelShape::new(OVERLAPPING_HALF_BLOCKS),
            VoxelShape::new(LOWER_HALF_BLOCK),
            BooleanOp::And
        ));
    }

    #[test]
    fn join_is_not_empty_rejects_disjoint_and() {
        assert!(!join_is_not_empty(
            VoxelShape::new(LOWER_HALF_BLOCK),
            VoxelShape::new(UPPER_HALF_BLOCK),
            BooleanOp::And
        ));
    }

    #[test]
    fn join_is_not_empty_detects_only_first_remainder() {
        assert!(join_is_not_empty(
            VoxelShape::FULL,
            VoxelShape::new(LOWER_HALF_BLOCK),
            BooleanOp::OnlyFirst
        ));
    }

    #[test]
    fn join_unoptimized_boxes_materializes_only_second_remainder() {
        let remainder = join_unoptimized_boxes(
            VoxelShape::new(LOWER_HALF_BLOCK),
            VoxelShape::FULL,
            BooleanOp::OnlySecond,
        );

        assert_eq!(remainder, vec![Aabb::new(0.0, 0.5, 0.0, 1.0, 1.0, 1.0)]);
    }

    #[test]
    fn shape_full_block_accepts_tiled_boxes() {
        assert!(is_shape_full_block(VoxelShape::new(SPLIT_FULL_BLOCK)));
    }

    #[test]
    fn shape_full_block_rejects_partial_boxes() {
        assert!(!is_shape_full_block(VoxelShape::new(LOWER_HALF_BLOCK)));
    }

    #[test]
    fn offset_shape_full_block_rejects_shifted_full_block() {
        assert!(is_offset_shape_full_block(
            OffsetVoxelShape::without_offset(VoxelShape::FULL)
        ));
        assert!(!is_offset_shape_full_block(OffsetVoxelShape::new(
            VoxelShape::FULL,
            Point::new(0.25, 0.0, 0.0)
        )));
    }

    #[test]
    fn offset_shape_full_block_rejects_z_gap_after_offset() {
        assert!(!is_offset_shape_full_block(OffsetVoxelShape::new(
            VoxelShape::new(Z_GAPPED_BLOCK_WITH_OFFSET),
            Point::new(0.0, 0.0, 0.1)
        )));
    }

    #[test]
    fn zero_volume_boxes_are_empty() {
        assert!(VoxelShape::new(ZERO_VOLUME_BOX).is_empty());
        assert!(!join_is_not_empty(
            VoxelShape::new(ZERO_VOLUME_BOX),
            VoxelShape::FULL,
            BooleanOp::And
        ));
    }

    #[test]
    fn face_full_accepts_union_covering_face() {
        assert!(is_face_full(
            VoxelShape::new(QUADRANT_TOP_FACE),
            Direction::Up
        ));
    }

    #[test]
    fn dynamic_face_support_uses_union_at_the_block_boundary() {
        assert!(is_block_local_face_sturdy(
            QUADRANT_TOP_FACE,
            Direction::Up,
            SupportType::Full,
        ));

        let beyond_east_face = [&Aabb::new(1.25, 0.0, 0.0, 2.25, 1.0, 1.0)];
        assert!(!is_block_local_face_sturdy(
            &beyond_east_face,
            Direction::East,
            SupportType::Full,
        ));
    }

    #[test]
    fn face_full_rejects_union_with_gap() {
        assert!(!is_face_full(
            VoxelShape::new(GAPPED_TOP_FACE),
            Direction::Up
        ));
    }

    #[test]
    fn offset_face_full_rejects_shifted_top_face() {
        assert!(!is_offset_face_full(
            OffsetVoxelShape::new(VoxelShape::FULL, Point::new(0.25, 0.0, 0.0)),
            Direction::Up
        ));
    }

    #[test]
    fn face_full_accepts_vanilla_azalea_top_shape() {
        assert!(is_face_full(
            VoxelShape::new(VANILLA_AZALEA_SHAPE),
            Direction::Up
        ));
    }

    #[test]
    fn rigid_support_accepts_border_ring_covered_by_multiple_boxes() {
        assert!(is_face_rigid_supported(
            VoxelShape::new(RIGID_TOP_RING),
            Direction::Up
        ));
    }

    #[test]
    fn rigid_support_rejects_center_panel_without_border_ring() {
        assert!(!is_face_rigid_supported(
            VoxelShape::new(RIGID_CENTER_PANEL),
            Direction::Up
        ));
    }

    #[test]
    fn rigid_support_rejects_side_border_ring_without_full_face() {
        assert!(!is_face_rigid_supported(
            VoxelShape::new(RIGID_WEST_FACE_RING),
            Direction::West
        ));
    }
}
