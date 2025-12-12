use crate::tool::Tool;

struct Server {
    tools: Vec<Box<dyn Tool>>,
}
