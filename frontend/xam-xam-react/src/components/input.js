import React, { useState }  from 'react';

export function InputWithButton(props) {
    const [input, setInput] = useState("");

    function input_press(event,input) {
        props.input_callback(input);
        event.preventDefault();
        event.stopPropagation();
        setInput('');
    }

    return <div className="input-group">
                <input type={props.type || "text"} className="form-control" value={input} name="input" onChange={(e) => setInput(e.target.value)} placeholder={props.valuePlaceholder || "" } required/>
                <div className="input-group-btn">
                    <button className="btn btn-default" type="submit" onClick={(event) => input_press(event,input)}>
                        {props.name}
                    </button>
                </div>
            </div>;
}