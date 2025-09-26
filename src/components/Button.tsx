import "./Button.styles.css";
import { MouseEventHandler, useState } from "react";

interface Props {
    text: string
    onClick?: MouseEventHandler<HTMLButtonElement>
}

export default function Button(props: Props) {
    const [isMouseDown, setIsMouseDown] = useState<boolean>(false);

    return <button
        className="btn"
        onMouseDown={() => setIsMouseDown(true)}
        onMouseUp={() => setIsMouseDown(false)}
        onClick={props.onClick}
        >
        {props.text}
        </button>;
}
