use std::collections::HashSet;
const EXTENDER_STYLE: &str = r#"
  extender-l{
    display:block;
    width: calc(100% + var(--pr) + var(--pl));
    margin-inline-start: calc(0px - var(--pl));
    margin-inline-end: calc(0px - var(--pr));
  }
"#;

const EXTENDER_SCREEN_STYLE: &str = r#"
  extender-l[layout~=screen]{
    width: 100vw;
    position: relative;
    margin-left: -50vw;
    margin-right: -50vw;
    left: 50%;
    right: 50%;
  }
  "#;

const EXTENDER_KEEP_CENTER_STYLE: &str = r#"
  extender-l[layout~="keep-center"] > *{
    box-sizing: content-box;
    max-inline-size: var(--center-max-width);
    margin-inline: auto;
  }
  "#;

const EXTENDER_KEEP_P_STYLE: &str = r#"
  extender-l[layout~="keep-p"] {
    padding-right: var(--pr);
    padding-left: var(--pl);
  }
"#;

const EXTENDER_KEEP_PL_STYLE: &str = r#"
  extender-l[layout~="keep-pl"] {
    padding-left: var(--pl);
    padding-right: unset;
  }
"#;

const EXTENDER_KEEP_PR_STYLE: &str = r#"
  extender-l[layout~="keep-pr"] {
    padding-right: var(--pr);
    padding-left: unset;
  }
"#;

pub fn extender_css(
    screen: bool,
    keep_center: bool,
    keep_p: bool,
    keep_pl: bool,
    keep_pr: bool,
    set: &mut HashSet<String>,
) {
    set.insert(EXTENDER_STYLE.to_string());
    if screen {
        set.insert(EXTENDER_SCREEN_STYLE.to_string());
    }
    if keep_center {
        set.insert(EXTENDER_KEEP_CENTER_STYLE.to_string());
    }
    if keep_p {
        set.insert(EXTENDER_KEEP_P_STYLE.to_string());
    }
    if keep_pl {
        set.insert(EXTENDER_KEEP_PL_STYLE.to_string());
    }
    if keep_pr {
        set.insert(EXTENDER_KEEP_PR_STYLE.to_string());
    }
}
