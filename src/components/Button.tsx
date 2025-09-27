import "./Button.styles.css";
import { MouseEventHandler } from "react";

interface Props {
    text: string
    onClick?: MouseEventHandler<HTMLButtonElement>
}

export default function Button(props: Props) {
    return <button
        className="btn"
        onClick={props.onClick}
        >
        {props.text}
        </button>;
}
