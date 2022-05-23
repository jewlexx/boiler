use esize::ENum;
use strum::{Display, EnumIter};

#[derive(Debug, Copy, Clone, ENum, Display, EnumIter)]
pub enum BoilerOpts {
    File,
    Project,
}
