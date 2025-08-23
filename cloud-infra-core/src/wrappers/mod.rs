// TODO documentation on how to create these!

/// A String that only contains letters, numbers and underscores (in any position)
/// Preferably create this wrapper by calling the proc macro 
/// `string_with_only_alpha_numerics_and_underscores` 
/// as that macro will check the correctness of your input 
#[derive(Debug, Clone)]
pub struct StringWithOnlyAlphaNumericsAndUnderscores(pub String);
#[derive(Debug, Copy, Clone)]
pub struct NonZeroNumber(pub u32);
#[derive(Debug, Copy, Clone)]
pub struct Memory(pub u16);
#[derive(Debug, Copy, Clone)]
pub struct Timeout(pub u16);
#[derive(Debug, Clone)]
pub struct EnvVarKey(pub String);
#[derive(Debug, Clone)]
pub struct ZipFile(pub String);
