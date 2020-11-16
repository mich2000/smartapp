import React, { useState, useEffect }  from 'react';
import Popup from 'reactjs-popup';
import api_functions from '../../api';

export const InputStorageDialog = (props) => {
    const types = Object.freeze({
        Other : 'Other',
        Closet : 'Closet',
        Fridge : 'Fridge',
        Freezer : 'Freezer'
    });
    const [name, setName] = useState('');
    const [type, setType] = useState(types.Other);

    useEffect(() => {
        fetch(api_functions.get_business_api() + '/storages', api_functions.method_get())
        .then((api_call) => {
            if(api_call.status === 200) {
                api_call.json()
                .then((json) => props.set_storage_list(json.storages))
                .catch((e) => {
                    console.error(`Could not send through the request. error: ${e}`);
                });
            }
        }).catch((e) => {
            console.error(`Could not send through the request. error: ${e}`);
        });
    })

    function add_storage(event) {
        event.preventDefault();
        event.stopPropagation();
        if(name === '') {
            alert("A name cannot be empty.");
            return;
        }
        let options = api_functions.method_post();
        options.body = JSON.stringify({
            name: name,
            kind: type
        });
        fetch(api_functions.get_business_api() + '/storage', options)
        .then((api_call) => {
            if(api_call.status === 200) {
                props.add_storage({
                    name: name,
                    kind: type
                });
                setType(types.Other);
                setName('');
            }
        }).catch((e) => {
            console.error(`Could not send through the request. error: ${e}`);
        });
    }

    return (
        <Popup trigger={<button className="btn btn-primary modal-input">Add storage</button>} modal nested>
            {
                close => (
                    <div className="modal-dialog">
                        <div className="modal-header">Add storage</div>
                        <div className="modal-content">
                            <input className="modal-input form-control" type="text" value={name} placeholder="Enter the storage name" onChange={(e) => setName(e.target.value)}/>
                            <select className="modal-input form-control" value={type} onChange={(e) => setType(e.target.value)}>
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