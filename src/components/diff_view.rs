use super::*;
use crate::crates::{CrateInfo, CrateResponse, CrateSource, VersionInfo};
use crate::router::*;
use similar::{ChangeTag, TextDiff};
use std::sync::Arc;
use yew::prelude::*;
use yew::suspense::*;
use yew_icons::{Icon as YewIcon, IconId};

#[derive(Properties, PartialEq, Clone)]
pub struct SourceViewProps {
    pub info: Arc<CrateResponse>,
    pub left: Arc<CrateSource>,
    pub right: Arc<CrateSource>,
    pub path: String,
}

#[function_component]
pub fn SourceView(props: &SourceViewProps) -> Html {
    let left = props
        .left
        .files
        .get(&props.path)
        .map(|s| s.as_str())
        .unwrap_or("")
        .to_string();
    let right = props
        .right
        .files
        .get(&props.path)
        .map(|s| s.as_str())
        .unwrap_or("")
        .to_string();
    let navigator = use_navigator().unwrap();
    let onselect = {
        let name = props.info.krate.id.clone();
        let left = props.left.version.num.clone();
        let right = props.right.version.num.clone();
        move |path: String| {
            navigator.push(&Route::File {
                name: name.clone(),
                left: left.clone(),
                right: right.clone(),
                path,
            })
        }
    };
    html! {
        <>
        <div style="width: 200px;">
        <FileTree
            info={props.info.clone()}
            left={props.left.clone()}
            right={props.right.clone()}
            path={props.path.clone()}
            {onselect}
        />
        </div>
        <DiffView {left} {right} path={props.path.clone()} />
        </>
    }
}

#[derive(Properties, PartialEq, Clone)]
pub struct DiffViewProps {
    pub path: String,
    pub left: String,
    pub right: String,
}

#[function_component]
pub fn DiffView(props: &DiffViewProps) -> Html {
    let diff = TextDiff::from_lines(&props.left, &props.right);
    html! {
        <>
        <p>{"Diff"}</p>
        {
            diff.iter_all_changes().map(|change| {
                let sign = match change.tag() {
                    ChangeTag::Delete => "-",
                    ChangeTag::Insert => "+",
                    ChangeTag::Equal => " ",
                };
                html!{ <p>{ format!("{sign}{change}") } </p> }
            }).collect::<Html>()
        }
        </>
    }
}
