macro_rules! prop_flag {
    ($(#[$attr:meta] $Ident:ident = $value:expr,)*) => {
        $(
            #[$attr]
            #[allow(unused, non_upper_case_globals)]
            pub const $Ident: i64 = $value;
        )*
    };
}

prop_flag! {
    /// s
    None = 0,
    /// Property is user-settable in the editor.
    Edit							= 0x0000000000000001,
    /// This is a constant function parameter
    ConstParm						= 0x0000000000000002,
    /// This property can be read by blueprint code
    BlueprintVisible				= 0x0000000000000004,
    /// Object can be exported with actor.
    ExportObject					= 0x0000000000000008,
    /// This property cannot be modified by blueprint code
    BlueprintReadOnly				= 0x0000000000000010,
    /// Property is relevant to network replication.
    Net								= 0x0000000000000020,
    /// Indicates that elements of an array can be modified, but its size cannot be changed.
    EditFixedSize					= 0x0000000000000040,
    /// Function/When call parameter.
    Parm							= 0x0000000000000080,
    /// Value is copied out after function call.
    OutParm							= 0x0000000000000100,
    /// memset is fine for construction
    ZeroConstructor					= 0x0000000000000200,
    /// Return value.
    ReturnParm						= 0x0000000000000400,
    /// Disable editing of this property on an archetype/sub-blueprint
    DisableEditOnTemplate			= 0x0000000000000800,
    /// Object property can never be null
    NonNullable						= 0x0000000000001000,
    /// Property is transient: shouldn't be saved or loaded, except for Blueprint CDOs.
    Transient   					= 0x0000000000002000,
    /// Property should be loaded/saved as permanent profile.
    Config      					= 0x0000000000004000,
    /// Parameter must be linked explicitly in blueprint. Leaving the parameter out results in a compile error.
    RequiredParm					= 0x0000000000008000,
    /// Disable editing on an instance of this class
    DisableEditOnInstance			= 0x0000000000010000,
    /// Property is uneditable in the editor.
    EditConst   					= 0x0000000000020000,
    /// Load config from base class, not subclass.
    GlobalConfig					= 0x0000000000040000,
    /// Property is a component references.
    InstancedReference				= 0x0000000000080000,
    /// Property saves objects in separate files, breaks hard links and reload them based on discovery.
    ExperimentalExternalObjects		= 0x0000000000100000,
    /// Property should always be reset to the default value during any type of duplication (copy/paste, binary duplication, etc.)
    DuplicateTransient				= 0x0000000000200000,
    //								= 0x0000000000400000,
    //   							= 0x0000000000800000,
    /// Property should be serialized for save games, this is only checked for game-specific archives with ArIsSaveGame
    SaveGame						= 0x0000000001000000,
    /// Hide clear button.
    NoClear							= 0x0000000002000000,
    /// Property is defined on an interface and does not include a useful Offset_Internal.
    Virtual							= 0x0000000004000000,
    /// Value is passed by reference; CPF_OutParam and CPF_Param should also be set.
    ReferenceParm					= 0x0000000008000000,
    /// MC Delegates only.  Property should be exposed for assigning in blueprint code
    BlueprintAssignable				= 0x0000000010000000,
    /// Property is deprecated.  Read it from an archive, but don't save it.
    Deprecated  					= 0x0000000020000000,
    /// If this is set, then the property can be memcopied instead of CopyCompleteValue / CopySingleValue
    IsPlainOldData					= 0x0000000040000000,
    /// Not replicated. For non replicated properties in replicated structs
    RepSkip							= 0x0000000080000000,
    /// Notify actors when a property is replicated
    RepNotify						= 0x0000000100000000,
    /// interpolatable property for use with cinematics
    Interp							= 0x0000000200000000,
    /// Property isn't transacted
    NonTransactional				= 0x0000000400000000,
    /// Property should only be loaded in the editor
    EditorOnly						= 0x0000000800000000,
    /// No destructor
    NoDestructor					= 0x0000001000000000,
    //								= 0x0000002000000000,
    /// Only used for weak pointers, means the export type is autoweak
    AutoWeak						= 0x0000004000000000,
    /// Property contains component references.
    ContainsInstancedReference		= 0x0000008000000000,
    /// asset instances will add properties with this flag to the asset registry automatically
    AssetRegistrySearchable			= 0x0000010000000000,
    /// The property is visible by default in the editor details view
    SimpleDisplay					= 0x0000020000000000,
    /// The property is advanced and not visible by default in the editor details view
    AdvancedDisplay					= 0x0000040000000000,
    /// property is protected from the perspective of script
    Protected						= 0x0000080000000000,
    /// MC Delegates only.  Property should be exposed for calling in blueprint code
    BlueprintCallable				= 0x0000100000000000,
    /// MC Delegates only.  This delegate accepts (only in blueprint) only events with BlueprintAuthorityOnly.
    BlueprintAuthorityOnly			= 0x0000200000000000,
    /// Property shouldn't be exported to text format (e.g. copy/paste)
    TextExportTransient				= 0x0000400000000000,
    /// Property should only be copied in PIE
    NonPIEDuplicateTransient		= 0x0000800000000000,
    /// Property is exposed on spawn
    ExposeOnSpawn					= 0x0001000000000000,
    /// A object referenced by the property is duplicated like a component. (Each actor should have an own instance.)
    PersistentInstance				= 0x0002000000000000,
    /// Property was parsed as a wrapper class like TSubclassOf<T>, FScriptInterface etc., rather than a USomething*
    UObjectWrapper					= 0x0004000000000000,
    /// This property can generate a meaningful hash value.
    HasGetValueTypeHash				= 0x0008000000000000,
    /// Public native access specifier
    NativeAccessSpecifierPublic		= 0x0010000000000000,
    /// Protected native access specifier
    NativeAccessSpecifierProtected	= 0x0020000000000000,
    /// Private native access specifier
    NativeAccessSpecifierPrivate	= 0x0040000000000000,
    /// Property shouldn't be serialized, can still be exported to text
    SkipSerialization				= 0x0080000000000000,
    /// Property is a TObjectPtr<T> instead of a USomething*. Need to differentiate between TObjectclassOf and TObjectPtr
    TObjectPtr						= 0x0100000000000000,
    /// ****Experimental*** Property will use different logic to serialize knowing what changes are done against its default use the overridable information provided by the overridable manager on the object
    ExperimentalOverridableLogic	= 0x0200000000000000,
    /// ****Experimental*** Property should never inherit from the parent when using overridable serialization
    ExperimentalAlwaysOverriden		= 0x0400000000000000,
    /// ****Experimental*** Property should never be overridden when using overridable serialization
    ExperimentalNeverOverriden		= 0x0800000000000000,
    /// Enables the instancing graph self referencing logic, delegates and verse function are already using this
    AllowSelfReference				= 0x1000000000000000,
}
