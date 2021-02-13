import React,{useState, useEffect} from 'react';
import Popup from 'reactjs-popup';
import {StorageType} from '../../enums';
import {PenIcon} from '../../icon';

export function EditStoragePopup(props) {
    const [name, setName] = useState(props.item[0]);
    const [type, setType] = useState(props.item[1]);
    
    useEffect(() => {
        setName(props.item[0]);
        setType(props.item[1]);
    },[props.item]);

    function edit_storage(event) {
        event.preventDefault();
        event.stopPropagation();
        props.edit_storage({
            storage_name: props.item[0],
            new_storage_name : (name !== '')? name : null,
            new_kind : type
        });
    }
    
    return <Popup trigger={<button className="badge badge-pill badge-secundary m-2" value={props.item} type="button">
            <PenIcon/>
        </button>}
        modal nested>
            {
                close => <div className="modal-dialog">
                    <div className="modal-header">Edit {props.item[0]}</div>
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
                        <button className="btn btn-primary modal-input" onClick={(e) => {edit_storage(e); close();}}>
                            Edit storage
                        </button>
                    </div>
                </div>
            }
        </Popup>;
}