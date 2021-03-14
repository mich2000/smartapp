import React, { useState }  from 'react';
import Popup from 'reactjs-popup';
import api_functions from '../../api';
import {StorageType} from '../../enums';
import {showError} from '../../toast';
import {PlusIcon} from '../../icon';

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
                api_call.text()
                .then(text => {
                    if(text !== 'No internet connection') {
                        props.add_storage({
                            name: name,
                            kind: type
                        });
                        setType(StorageType.Other);
                        setName('');
                    } else {
                        showError(text);
                    }
                });
            } else {
                api_call.text()
                .then(err => showError(err));
            }
        }).catch(() => showError('No internet connection'));
    }

    return <Popup trigger={
        <button className="btn btn-primary modal-input">
            <PlusIcon/>
        </button>} modal nested>
            {
                close => <div className="modal-dialog">
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
            }
        </Popup>;
}