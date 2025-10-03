import "./Button.styles.css";
import { MouseEventHandler } from "react";

interface Props {
    text: string
    onClick?: MouseEventHandler<HTMLButtonElement>
    color?: string
}

export default function Button(props: Props) {
    return <button
        className="btn"
        onClick={props.onClick}
        style={{
            color: props.color ? props.color : "#f1f1f1"
        }}
        >
        {props.text}
        </button>;
}
