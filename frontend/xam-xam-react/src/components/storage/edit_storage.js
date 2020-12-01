import React,{useState} from 'react';
import Popup from 'reactjs-popup';
import {storage_type} from '../../enums';

export function EditStoragePopup(props) {
    const [name, setName] = useState(props.item[0]);
    const [type, setType] = useState(props.item[1]);

    function edit_storage(event) {
        event.preventDefault();
        event.stopPropagation();
        props.edit_storage({
            storage_name: props.item[0],
            new_storage_name : (name !== '')? name : null,
            new_kind : type
        });
    }

    return (
        <Popup trigger={<button className="badge badge-pill badge-secundary m-2" value={props.item} type="button">Edit</button>}
        modal nested>
            {
                close =>
                <div className="modal-dialog">
                    <div className="modal-header">Edit {props.item[0]}</div>
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
                        <button className="btn btn-primary modal-input" onClick={(e) => {edit_storage(e); close();}}>
                            Edit storage
                        </button>
                    </div>
                </div>
            }
        </Popup>
    );
}