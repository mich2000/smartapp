import React, { useState }  from 'react';
import Popup from 'reactjs-popup';
import api_functions from '../../api';

export function InputStorageDialog(props) {
    const types = Object.freeze({
        Other : 'other',
        Closet : 'closet',
        Fridge : 'fridge',
        Freezer : 'freezer'
    });
    const [name, setName] = useState('');
    const [type, setType] = useState(types.Other);

    function add_storage(event) {
        event.preventDefault();
        event.stopPropagation();
        if(name === '') {
            alert("A name cannot be empty.");
            return;
        }
        console.log('buton');
        let options = api_functions.method_post();
        options.body = JSON.stringify({
            name: name,
            kind: type
        });
        fetch(api_functions.get_business_api() + '/storage', options)
        .then((api_call) => {
            if(api_call.status === 200) {
                console.log("A storage has been added");
                props.add_storage({
                    name: name,
                    kind: type
                });
                setType({
                    name: '',
                    kind: types.Other
                });
            }
        }).catch((e) => {
            console.error(`Could not send through the request. error: ${e}`);
        });
    }

    return (
        <Popup trigger={<button className="button"> Open Modal </button>} modal nested>
            {
                close => (
                    <div className="modal-dialog">
                        <div className="modal-header">Add storage</div>
                        <div className="modal-content">
                            <input className="modal-input" type="text" value={name} placeholder="Enter the storage name" onChange={(e) => setName(e.target.value)}/>
                            <select className="modal-input" value={type} onChange={(e) => setType(e.target.value)}>
                                {Object.keys(types).map(key => (
                                    <option key={key} value={key}>
                                        {types[key]}
                                    </option>
                                    )
                                )}
                            </select>
                            <button className="btn btn-primary modal-input" onClick={(e) => {add_storage(e);close();}}>
                                Add storage
                            </button>
                        </div>
                    </div>
                )
            }
        </Popup>
    );
}