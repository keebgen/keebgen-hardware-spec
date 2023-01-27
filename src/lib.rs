/// A specification of a keyboard with its required context
// TODO: make this into an actual type?
#[derive(Debug, knuffel::Decode)]
pub enum KeyboardSpec {
    /// The keyboard being specified
    Keyboard(Keyboard),
    /// All of the component definitions for this keyboard's components
    Definition(Definitions),
}

/// A collection of definitions for a keyboard
#[derive(Debug, knuffel::Decode)]
pub struct Definitions {
    /// All of the definitions needed for a keyboard
    pub definitions: Vec<Definition>,
}

/// The hardware definition of a keyboard
#[derive(Debug, knuffel::Decode)]
pub struct Keyboard {
    /// The name of the keyboard
    #[knuffel(node_name)]
    pub name: String,
    /// The name of the keyboard's designer
    #[knuffel(property)]
    pub designer: Option<String>,
    /// When this keyboard was designed
    // TODO: Use a real date type
    #[knuffel(property)]
    pub designed: Option<String>,
    /// The components on this keyboard
    #[knuffel(children)]
    pub components: Vec<Component>,
}

/// An inline or remote definition
#[derive(Debug, knuffel::Decode)]
pub enum Definition {
    /// A definition written locally in this keyboard definition
    Inline(InlineDefinition),
    /// A definition pulled from another keyboard spec
    Remote(RemoteDefinition),
}

/// A definition written locally in this keyboard definition
#[derive(Debug, knuffel::Decode)]
pub struct InlineDefinition {
    /// The name of the definition
    #[knuffel(node_name)]
    pub name: String,
    /// What kind of component
    #[knuffel(argument, default)]
    pub kind: ComponentKind,
    /// The inputs available on this component
    #[knuffel(children)]
    pub inputs: Vec<Input>,
}

/// An input to a component
#[derive(Debug, knuffel::Decode)]
pub struct Input {
    /// The name of the input
    #[knuffel(node_name)]
    pub name: String,
    /// The kind of the input
    #[knuffel(property)]
    pub kind: InputKind,
}

/// A possible kind of input
#[derive(Debug, knuffel::DecodeScalar)]
pub enum InputKind {
    /// A ground pin
    Ground,
    /// A general-purpose IO pin
    Io,
}

/// A definition pulled from another keyboard spec
#[derive(Debug, knuffel::Decode)]
pub struct RemoteDefinition {
    /// The name of the definition
    #[knuffel(node_name)]
    pub name: String,
    /// The URL of the git repository to pull from
    #[knuffel(argument)]
    pub repository: String,
    /// The path to the definition file within
    #[knuffel(argument)]
    pub path: String,
    /// The name of the definition in the other definition.
    /// If omitted, the name is assumed to be the one set as
    /// this definition's `name`.
    #[knuffel(argument)]
    pub target: Option<String>,
}

/// The different kinds of components
// TODO: is this needed? Seems needed for being able
// to distinguish switches from the perspective of "info.json"
#[derive(Debug, Default, knuffel::DecodeScalar)]
pub enum ComponentKind {
    /// A microcontroller
    Controller,
    /// A switch
    #[default]
    Switch,
    /// Some other peripheral
    Peripheral,
}

/// Any component on a keyboard
#[derive(Debug, knuffel::Decode)]
pub struct Component {
    /// The name of the definition for this component
    #[knuffel(argument)]
    pub definition: String,
    /// How much this component should be rotated
    #[knuffel(property)]
    pub rotation: Option<u64>,
    /// The location of the given component
    #[knuffel(child)]
    pub location: Location,
    /// Where each output of this node goes to
    #[knuffel(children)]
    pub outputs: Vec<Output>,
}

/// The location of a component
#[derive(Debug, knuffel::Decode)]
pub struct Location {
    /// The x position
    pub x: u64,
    /// The y position
    pub y: u64,
}

/// An output from a component
// TODO: add a simpler, tuple-based variant?
#[derive(Debug, knuffel::Decode)]
pub struct Output {
    /// The index of another component to target
    #[knuffel(property)]
    target: u64,
    /// The index of the input within the component
    #[knuffel(property)]
    input: u64,
}
