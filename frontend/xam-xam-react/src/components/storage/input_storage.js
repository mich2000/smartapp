import React, { useState }  from 'react';
import Popup from 'reactjs-popup';
import api_functions from '../../api';
import {StorageType} from '../../enums';
import {showError} from '../../toast';

export const InputStorageDialog = (props) => {
    const [name, setName] = useState('');
    const [type, setType] = useState(StorageType.Other);

    function add_storage(event) {
        event.preventDefault();
        event.stopPropagation();
        let options = api_functions.method_post();
        options.body = JSON.stringify({ name : name, kind : type });
        fetch(api_functions.get_business_api() + '/storage', options)
        .then((api_call) => {
            if(api_call.status === 200) {
                props.add_storage({
                    name: name,
                    kind: type
                });
                setType(StorageType.Other);
                setName('');
            } else {
                api_call.text()
                .then(err => showError(err));
            }
        }).catch((e) =>  showError(`Could not send through the request. error: ${e}`));
    }

    return (
        <Popup trigger={<button className="btn btn-primary modal-input">
            <svg width="1em" height="1em" viewBox="0 0 16 16" className="bi bi-plus svg-plus" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
                <path fillRule="evenodd" d="M8 4a.5.5 0 0 1 .5.5v3h3a.5.5 0 0 1 0 1h-3v3a.5.5 0 0 1-1 0v-3h-3a.5.5 0 0 1 0-1h3v-3A.5.5 0 0 1 8 4z"/>
            </svg>
        </button>} modal nested>
            {
                close => (
                    <div className="modal-dialog">
                        <div className="modal-header">Add storage</div>
                        <div className="modal-content">
                            <input className="modal-input form-control" type="text" value={name} placeholder="Enter the storage name" onChange={(e) => setName(e.target.value)}/>
                            <select className="modal-input form-control" value={type} onChange={(e) => setType(e.target.value)}>
                                {Object.keys(StorageType).map(key => (
                                    <option key={key} value={key}>
                                        {StorageType[key]}
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