import React,{useState, useEffect} from 'react';
import Popup from 'reactjs-popup';
import {StorageType} from '../../enums';

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
            <svg width="1em" height="1em" viewBox="0 0 16 16" className="bi bi-pencil" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
                <path fillRule="evenodd" d="M12.146.146a.5.5 0 0 1 .708 0l3 3a.5.5 0 0 1 0 .708l-10 10a.5.5 0 0 1-.168.11l-5 2a.5.5 0 0 1-.65-.65l2-5a.5.5 0 0 1 .11-.168l10-10zM11.207 2.5L13.5 4.793 14.793 3.5 12.5 1.207 11.207 2.5zm1.586 3L10.5 3.207 4 9.707V10h.5a.5.5 0 0 1 .5.5v.5h.5a.5.5 0 0 1 .5.5v.5h.293l6.5-6.5zm-9.761 5.175l-.106.106-1.528 3.821 3.821-1.528.106-.106A.5.5 0 0 1 5 12.5V12h-.5a.5.5 0 0 1-.5-.5V11h-.5a.5.5 0 0 1-.468-.325z"/>
            </svg>
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