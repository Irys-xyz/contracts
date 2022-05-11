use serde::Serialize;

#[derive(Debug, PartialEq, Serialize)]
#[serde(untagged)]
pub enum HandlerResult<State, QueryResponseMsg> {
    NewState(State),
    QueryResponse(QueryResponseMsg),
}
