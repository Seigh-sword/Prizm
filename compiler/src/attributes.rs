// Headers and Attributes System for Prizm

pub struct AttributeID {
    pub id: u32,
    pub name: String,
    pub header: String,
}

impl AttributeID {
    pub fn new(id: u32, name: &str, header: &str) -> Self {
        AttributeID {
            id,
            name: name.to_string(),
            header: header.to_string(),
        }
    }
}

// File Operations Header
pub mod file {
    use super::AttributeID;

    pub const CREATE: u32 = 101;
    pub const DELETE: u32 = 102;
    pub const MOVE: u32 = 103;
    pub const REPLACE: u32 = 104;
    pub const MODIFY: u32 = 105;
    pub const ACCESS: u32 = 106;

    pub fn get_attribute(attr_id: u32) -> Option<&'static str> {
        match attr_id {
            CREATE => Some("create"),
            DELETE => Some("delete"),
            MOVE => Some("move"),
            REPLACE => Some("replace"),
            MODIFY => Some("modify"),
            ACCESS => Some("access"),
            _ => None,
        }
    }
}

// Math Operations Header
pub mod math {
    pub const ADD: u32 = 201;
    pub const SUBTRACT: u32 = 202;
    pub const MULTIPLY: u32 = 203;
    pub const DIVIDE: u32 = 204;
    pub const MODULO: u32 = 205;
    pub const RANDOM: u32 = 206;
    pub const POWER: u32 = 207;

    pub fn get_attribute(attr_id: u32) -> Option<&'static str> {
        match attr_id {
            ADD => Some("add"),
            SUBTRACT => Some("subtract"),
            MULTIPLY => Some("multiply"),
            DIVIDE => Some("divide"),
            MODULO => Some("modulo"),
            RANDOM => Some("random"),
            POWER => Some("power"),
            _ => None,
        }
    }
}

// Control Flow Header
pub mod control {
    pub const IF: u32 = 301;
    pub const ELSE: u32 = 302;
    pub const ELSE_IF: u32 = 303;
    pub const LOOP: u32 = 304;
    pub const LOOP_UNTIL: u32 = 305;
    pub const REPEAT: u32 = 306;
    pub const BREAK: u32 = 307;

    pub fn get_attribute(attr_id: u32) -> Option<&'static str> {
        match attr_id {
            IF => Some("if"),
            ELSE => Some("else"),
            ELSE_IF => Some("else if"),
            LOOP => Some("loop"),
            LOOP_UNTIL => Some("loop until"),
            REPEAT => Some("repeat"),
            BREAK => Some("break"),
            _ => None,
        }
    }
}

// Function Definition Header
pub mod function {
    pub const DEFINE: u32 = 401;
    pub const CALL: u32 = 402;
    pub const RETURN: u32 = 403;

    pub fn get_attribute(attr_id: u32) -> Option<&'static str> {
        match attr_id {
            DEFINE => Some("define"),
            CALL => Some("call"),
            RETURN => Some("return"),
            _ => None,
        }
    }
}

// HTTP Operations Header
pub mod http {
    pub const GET: u32 = 501;
    pub const POST: u32 = 502;
    pub const PUT: u32 = 503;
    pub const DELETE: u32 = 504;
    pub const HEADER: u32 = 505;

    pub fn get_attribute(attr_id: u32) -> Option<&'static str> {
        match attr_id {
            GET => Some("get"),
            POST => Some("post"),
            PUT => Some("put"),
            DELETE => Some("delete"),
            HEADER => Some("header"),
            _ => None,
        }
    }
}

// Variable Operations Header
pub mod var {
    pub const DECLARE: u32 = 601;
    pub const ASSIGN: u32 = 602;
    pub const ACCESS: u32 = 603;

    pub fn get_attribute(attr_id: u32) -> Option<&'static str> {
        match attr_id {
            DECLARE => Some("declare"),
            ASSIGN => Some("assign"),
            ACCESS => Some("access"),
            _ => None,
        }
    }
}


// UI Operations Header (GUI/Window Management)
pub mod ui {
    pub const WINDOW: u32 = 801;
    pub const BUTTON: u32 = 802;
    pub const TEXT: u32 = 803;
    pub const INPUT: u32 = 804;
    pub const LABEL: u32 = 805;
    pub const PANEL: u32 = 806;
    pub const EVENT: u32 = 807;
    pub const RENDER: u32 = 808;

    pub fn get_attribute(attr_id: u32) -> Option<&'static str> {
        match attr_id {
            WINDOW => Some("window"),
            BUTTON => Some("button"),
            TEXT => Some("text"),
            INPUT => Some("input"),
            LABEL => Some("label"),
            PANEL => Some("panel"),
            EVENT => Some("event"),
            RENDER => Some("render"),
            _ => None,
        }
    }
}

// Root Header (Super Powerful Operations)
pub mod root {
    pub const EXEC: u32 = 901;
    pub const SYSTEM: u32 = 902;
    pub const MEMORY: u32 = 903;
    pub const PROCESS: u32 = 904;
    pub const INTERRUPT: u32 = 905;
    pub const OPTIMIZE: u32 = 906;

    pub fn get_attribute(attr_id: u32) -> Option<&'static str> {
        match attr_id {
            EXEC => Some("exec"),
            SYSTEM => Some("system"),
            MEMORY => Some("memory"),
            PROCESS => Some("process"),
            INTERRUPT => Some("interrupt"),
            OPTIMIZE => Some("optimize"),
            _ => None,
        }
    }
}

// Data/JSON Operations Header (Prizm JSON Format)
pub mod data {
    pub const ENCODE: u32 = 1001;
    pub const DECODE: u32 = 1002;
    pub const PARSE: u32 = 1003;
    pub const STRINGIFY: u32 = 1004;
    pub const VALIDATE: u32 = 1005;
    pub const MERGE: u32 = 1006;

    pub fn get_attribute(attr_id: u32) -> Option<&'static str> {
        match attr_id {
            ENCODE => Some("encode"),
            DECODE => Some("decode"),
            PARSE => Some("parse"),
            STRINGIFY => Some("stringify"),
            VALIDATE => Some("validate"),
            MERGE => Some("merge"),
            _ => None,
        }
    }
}

// Time Operations Header
pub mod time {
    pub const NOW: u32 = 1101;
    pub const SLEEP: u32 = 1102;
    pub const FORMAT: u32 = 1103;
    pub const PARSE: u32 = 1104;
    pub const TIMER: u32 = 1105;
    pub const TIMESTAMP: u32 = 1106;

    pub fn get_attribute(attr_id: u32) -> Option<&'static str> {
        match attr_id {
            NOW => Some("now"),
            SLEEP => Some("sleep"),
            FORMAT => Some("format"),
            PARSE => Some("parse"),
            TIMER => Some("timer"),
            TIMESTAMP => Some("timestamp"),
            _ => None,
        }
    }
}

// Type System for Prizm
#[derive(Debug, Clone, PartialEq)]
pub enum PrizmType {
    Int,
    Float,
    String,
    Boolean,
    Array,
    Object,
    Null,
    Any,
}

impl PrizmType {
    pub fn to_string(&self) -> String {
        match self {
            PrizmType::Int => "int",
            PrizmType::Float => "float",
            PrizmType::String => "string",
            PrizmType::Boolean => "boolean",
            PrizmType::Array => "array",
            PrizmType::Object => "object",
            PrizmType::Null => "null",
            PrizmType::Any => "any",
        }.to_string()
    }

    pub fn from_string(s: &str) -> Option<PrizmType> {
        match s {
            "int" => Some(PrizmType::Int),
            "float" => Some(PrizmType::Float),
            "string" => Some(PrizmType::String),
            "boolean" | "bool" => Some(PrizmType::Boolean),
            "array" => Some(PrizmType::Array),
            "object" => Some(PrizmType::Object),
            "null" => Some(PrizmType::Null),
            "any" => Some(PrizmType::Any),
            _ => None,
        }
    }
}

pub mod builtins {
    pub const PRINT: u32 = 701;
    pub const PRINTLN: u32 = 702;

    pub fn get_attribute(attr_id: u32) -> Option<&'static str> {
        match attr_id {
            PRINT => Some("print"),
            PRINTLN => Some("println"),
            _ => None,
        }
    }
}