use com::{interface, IUnknown};

use winapi::shared::winerror::HRESULT;

#[interface(4FC333E3-C389-4C48-B108-7895B0AF21AD)]
pub trait ILocalFileManager: IUnknown {
    fn delete_local(&mut self) -> HRESULT;
}
