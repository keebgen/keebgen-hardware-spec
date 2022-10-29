use serde::{Deserialize, Serialize};
use time::Date;

/// The hardware definition of a keyboard
#[derive(Debug, Deserialize, Serialize)]
pub struct Keyboard {
    /// The name of the keyboard
    pub name: String,
    /// The name of the keyboard's designer
    pub designer: String,
    /// When this keyboard was designed
    #[serde(default)]
    pub design_date: Option<Date>,
    /// The components on this keyboard
    pub components: Vec<Component>,
    /// All of the component definitions for this keyboard's components
    pub definitions: Vec<Definition>,
}

/// An inline or remote definition
#[derive(Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum Definition {
    /// A definition written locally in this keyboard definition
    Inline(InlineDefinition),
    /// A definition pulled from another keyboard spec
    Remote(RemoteDefinition),
}

/// A definition written locally in this keyboard definition
#[derive(Debug, Deserialize, Serialize)]
pub struct InlineDefinition {
    /// The name of the definition
    pub name: String,
    /// What kind of component
    #[serde(default)]
    pub kind: ComponentKind,
    /// The inputs available on this component
    pub inputs: Vec<InputKind>,
}

/// An input to a component
#[derive(Debug, Deserialize, Serialize)]
pub struct Input {
    /// The name of the input
    pub name: String,
    /// The kind of the input
    pub kind: InputKind,
}

/// A possible kind of input
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum InputKind {
    /// A ground pin
    Ground,
    /// A general-purpose IO pin
    Io,
}

/// A definition pulled from another keyboard spec
#[derive(Debug, Deserialize, Serialize)]
pub struct RemoteDefinition {
    /// The name of the definition
    pub name: String,
    /// The URL of the git repository to pull from
    pub repository: String,
    /// The path to the definition file within
    pub path: String,
    /// The name of the definition in the other definition.
    /// If omitted, the name is assumed to be the one set as
    /// this definition's `name`.
    pub target: Option<String>,
}

/// The different kinds of components
// TODO: is this needed? Seems needed for being able
// to distinguish switches from the perspective of "info.json"
#[derive(Debug, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum ComponentKind {
    /// A microcontroller
    Controller,
    /// A switch
    #[default]
    Switch,
    /// An encoder
    Encoder,
    /// Some other peripheral
    Peripheral,
}

/// Any component on a keyboard
#[derive(Debug, Deserialize, Serialize)]
pub struct Component {
    /// The name of the definition for this component
    #[serde(alias = "def")]
    pub definition: String,
    /// Where each output of this node goes to
    pub outputs: Vec<Output>,
}

/// An output from a component
#[derive(Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum Output {
    Simple([u64; 2]),
    Full {
        /// The index of another component to target
        target: u64,
        /// The index of the input within the component
        input: u64,
    },
}
