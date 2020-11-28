import React, { useState }  from 'react';
import Popup from 'reactjs-popup';
import api_functions from '../../api';
import {storage_type} from '../../enums';

export const InputStorageDialog = (props) => {
    const [name, setName] = useState('');
    const [type, setType] = useState(storage_type.Other);

    function add_storage(event) {
        event.preventDefault();
        event.stopPropagation();
        if(name === '') { return; }
        let options = api_functions.method_post();
        options.body = JSON.stringify({ name : name, kind : type });
        fetch(api_functions.get_business_api() + '/storage', options)
        .then((api_call) => {
            if(api_call.status === 200) {
                props.add_storage({
                    name: name,
                    kind: type
                });
                setType(storage_type.Other);
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
                                {Object.keys(storage_type).map(key => (
                                    <option key={key} value={key}>
                                        {storage_type[key]}
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