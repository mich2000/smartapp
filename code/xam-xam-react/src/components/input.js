import React, { useState }  from 'react';

export function InputWithButton(props) {
    const [input, setInput] = useState("");

    return (
        <form onSubmit={(event) => props.input_callback(event,input)}>
            <div className="input-group">
                <input type={props.type || "text"} className="form-control" value={input} name="input" onChange={(e) => setInput( e.target.value )} placeholder={props.valuePlaceholder || "" } required/>
                <div className="input-group-btn">
                    <button className="btn btn-default" type="submit">
                        {props.name}
                    </button>
                </div>
            </div>
        </form>
    );
}