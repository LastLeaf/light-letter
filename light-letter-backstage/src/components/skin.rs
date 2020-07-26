use maomi::prelude::*;

skin!(pub(super) COMPONENTS = r#"
    @set main-color: #202020;
    @set main-border-color: #808080;
    @set main-background-color: #e7e7e7;

    .input {
        display: block;
        box-sizing: border-box;
        width: 100%;
        border: 1px solid main-border-color;
        background: main-background-color;
        color: main-color;
        padding: 3px 5px;
        line-height: 1.25em;
        font-size: 1em;
    }

    .button {
        box-sizing: border-box;
        border: 1px solid main-border-color;
        background: main-background-color;
        color: main-color;
        padding: 3px 5px;
        line-height: 1.25em;
    }
"#);
