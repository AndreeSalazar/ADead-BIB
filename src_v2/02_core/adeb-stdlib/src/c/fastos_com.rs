// ============================================================
// fastos_com.rs — COM (Component Object Model) support
// ============================================================
// Core COM types, GUIDs, and ole32.dll/oleaut32.dll functions
// Required foundation for ALL DirectX versions (DX9-DX12)
// ============================================================

// ── COM Functions (ole32.dll) ──
pub const COM_FUNCTIONS: &[&str] = &[
    "CoInitialize", "CoInitializeEx", "CoUninitialize",
    "CoCreateInstance", "CoGetClassObject",
    "CoTaskMemAlloc", "CoTaskMemRealloc", "CoTaskMemFree",
    "StringFromCLSID", "CLSIDFromString",
    "StringFromGUID2", "IIDFromString", "StringFromIID",
    "CoRegisterClassObject", "CoRevokeClassObject",
    "PropVariantClear",
    "OleInitialize", "OleUninitialize",
    "CoMarshalInterThreadInterfaceInStream",
    "CoGetInterfaceAndReleaseStream",
];

// ── COM Automation Functions (oleaut32.dll) ──
pub const OLEAUT_FUNCTIONS: &[&str] = &[
    "SysAllocString", "SysAllocStringLen",
    "SysFreeString", "SysStringLen", "SysStringByteLen",
    "SysReAllocString", "SysReAllocStringLen",
    "VariantInit", "VariantClear", "VariantCopy",
    "VariantChangeType", "VariantChangeTypeEx",
    "SafeArrayCreate", "SafeArrayCreateVector",
    "SafeArrayDestroy", "SafeArrayAccessData", "SafeArrayUnaccessData",
    "SafeArrayGetLBound", "SafeArrayGetUBound",
    "RegisterTypeLib",
];

// ── COM Types ──
pub const COM_TYPES: &[&str] = &[
    "HRESULT", "GUID", "IID", "CLSID", "REFIID", "REFCLSID", "REFGUID",
    "IUnknown", "IClassFactory",
    "BSTR", "VARIANT", "SAFEARRAY",
    "LPUNKNOWN", "LPVOID",
    "COINIT",
];

// ── COM Macros / Constants ──
pub const COM_MACROS: &[(&str, &str)] = &[
    ("S_OK", "((HRESULT)0x00000000L)"),
    ("S_FALSE", "((HRESULT)0x00000001L)"),
    ("E_FAIL", "((HRESULT)0x80004005L)"),
    ("E_INVALIDARG", "((HRESULT)0x80070057L)"),
    ("E_OUTOFMEMORY", "((HRESULT)0x8007000EL)"),
    ("E_NOINTERFACE", "((HRESULT)0x80004002L)"),
    ("E_POINTER", "((HRESULT)0x80004003L)"),
    ("E_NOTIMPL", "((HRESULT)0x80004001L)"),
    ("E_UNEXPECTED", "((HRESULT)0x8000FFFFL)"),
    ("E_ACCESSDENIED", "((HRESULT)0x80070005L)"),
    ("E_ABORT", "((HRESULT)0x80004004L)"),
    ("SUCCEEDED(hr)", "((HRESULT)(hr) >= 0)"),
    ("FAILED(hr)", "((HRESULT)(hr) < 0)"),
    ("COINIT_APARTMENTTHREADED", "0x2"),
    ("COINIT_MULTITHREADED", "0x0"),
    ("COINIT_DISABLE_OLE1DDE", "0x4"),
    ("COINIT_SPEED_OVER_MEMORY", "0x8"),
    ("CLSCTX_INPROC_SERVER", "0x1"),
    ("CLSCTX_INPROC_HANDLER", "0x2"),
    ("CLSCTX_LOCAL_SERVER", "0x4"),
    ("CLSCTX_ALL", "0x17"),
];

// ── Well-known GUIDs (Data1 values for quick identification) ──
pub const COM_KNOWN_IIDS: &[(&str, &str)] = &[
    ("IID_IUnknown", "{00000000-0000-0000-C000-000000000046}"),
    ("IID_IClassFactory", "{00000001-0000-0000-C000-000000000046}"),
];

pub fn is_com_symbol(name: &str) -> bool {
    COM_FUNCTIONS.contains(&name)
        || OLEAUT_FUNCTIONS.contains(&name)
        || COM_TYPES.contains(&name)
        || COM_MACROS.iter().any(|(n, _)| *n == name)
}
