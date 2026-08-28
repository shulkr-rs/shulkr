use shulkr_macros::Enumeration;

#[derive(Enumeration)]
pub enum EntityAnimation {
    SwingMainArm = 0,
    LeaveBed = 2,
    SwingOffhand = 3,
    CriticalEffect = 4,
    MagicalCriticalEffect = 5,
}
