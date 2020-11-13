import React,{useState} from 'react';
import Popup from 'reactjs-popup';

export function EditStoragePopup(props) {
    const types = Object.freeze({
        Other : 'Other',
        Closet : 'Closet',
        Fridge : 'Fridge',
        Freezer : 'Freezer'
    });
    const [name, setName] = useState('');
    const [type, setType] = useState(types.Other);

    function edit_storage(event) {
        event.preventDefault();
        event.stopPropagation();
        props.edit_storage({
            name: name,
            kind: type
        });
    }

    return (
        <Popup trigger={<button className="badge badge-pill badge-danger m-2" value={props.item} type="button">Edit</button>}
        modal nested>
            {
                close =>
                <div className="modal-dialog">
                    <div className="modal-header">Edit storage</div>
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
                        <button className="btn btn-primary modal-input" onClick={(e) => {edit_storage(e);close();}}>
                            Edit storage
                        </button>
                    </div>
                </div>
            }
        </Popup>
    );
}