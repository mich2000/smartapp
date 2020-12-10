import React, { useState }  from 'react';
import Popup from 'reactjs-popup';
import api_functions from '../../api';
import {ProductType} from '../../enums';

export const EditProductDialog = (props) => {
    const [name, setName] = useState(props.item_info[1]);
    const [amount, setAmount] = useState(parseInt(props.item_info[2]));
    const [date, setDate] = useState(props.item_info[3]);
    const [type, setType] = useState(props.item_info[4]);
    const id = props.item_info[0];

    function edit_product(event) {
        event.preventDefault();
        event.stopPropagation();
        props.edit_product({
            id : id,
            name : name,
            amount : parseInt(amount),
            date : date,
            kind : type
        });
    }
    
    return (
        <Popup trigger={<button className="btn btn-primary modal-input">
            <svg width="1em" height="1em" viewBox="0 0 16 16" className="bi bi-pencil" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
                <path fillRule="evenodd" d="M12.146.146a.5.5 0 0 1 .708 0l3 3a.5.5 0 0 1 0 .708l-10 10a.5.5 0 0 1-.168.11l-5 2a.5.5 0 0 1-.65-.65l2-5a.5.5 0 0 1 .11-.168l10-10zM11.207 2.5L13.5 4.793 14.793 3.5 12.5 1.207 11.207 2.5zm1.586 3L10.5 3.207 4 9.707V10h.5a.5.5 0 0 1 .5.5v.5h.5a.5.5 0 0 1 .5.5v.5h.293l6.5-6.5zm-9.761 5.175l-.106.106-1.528 3.821 3.821-1.528.106-.106A.5.5 0 0 1 5 12.5V12h-.5a.5.5 0 0 1-.5-.5V11h-.5a.5.5 0 0 1-.468-.325z"/>
            </svg>
        </button>} modal nested>
            {
                close => (
                    <div className="modal-dialog">
                        <div className="modal-header">Edit product</div>
                        <div className="modal-content">
                            <input className="modal-input form-control" type="text" value={name} placeholder="Enter the product name" onChange={(e) => setName(e.target.value)}/>
                            <input className="modal-input form-control" type="number" value={amount} placeholder="Enter the amount" onChange={(e) => setAmount(e.target.value)}/>
                            <input className="modal-input form-control" type="date" value={date} placeholder="Enter the expiration date" onChange={(e) => setDate(e.target.value)}/>
                            <select className="modal-input form-control" value={type} onChange={(e) => setType(e.target.value)}>
                                {Object.keys(ProductType).map(key => (
                                    <option key={key} value={key}>
                                        {ProductType[key]}
                                    </option>
                                    )
                                )}
                            </select>
                            <button className="btn btn-primary modal-input" onClick={(e) => {edit_product(e);close();}}>
                                Edit product
                            </button>
                        </div>
                    </div>
                )
            }
        </Popup>
    );
}