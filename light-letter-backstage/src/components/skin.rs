use maomi::prelude::*;

skin!(pub(super) COMPONENTS = r#"
    @set main-color: #333;
    @set main-border-color: #888;
    @set main-background-color: #e7e7e7;
    @set error-color: #e44;
    @set error-border-color: #d66;
    @set error-background-color: #fdd;
    @set warn-color: #962;
    @set warn-border-color: #b73;
    @set warn-background-color: #edc;
    @set info-color: #488;
    @set info-border-color: #6bb;
    @set info-background-color: #dee;

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

    .hint-area {
        position: fixed;
        max-width: 360px;
        right: 10px;
        top: 10px;
    }
    .hint {
        cursor: pointer;
        margin: 10px 10px 0 0;
        box-sizing: border-box;
        padding: 5px 10px;
        border-radius: 10px;
        color: main-color;
        border: 3px solid main-border-color;
        background: main-background-color;
    }
    .hint-error {
        color: error-color;
        border: 3px solid error-border-color;
        background: error-background-color;
    }
    .hint-warn {
        color: warn-color;
        border: 3px solid warn-border-color;
        background: warn-background-color;
    }
    .hint-info {
        color: info-color;
        border: 3px solid info-border-color;
        background: info-background-color;
    }
"#);
