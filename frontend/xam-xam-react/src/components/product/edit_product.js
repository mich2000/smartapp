import React, { useState, useEffect }  from 'react';
import Popup from 'reactjs-popup';
import {ProductType} from '../../enums';
import {showError} from '../../toast';
import {PenIcon} from '../../icon';

export const EditProductDialog = (props) => {
    const [name, setName] = useState(props.item_info[1]);
    const [amount, setAmount] = useState(parseInt(props.item_info[2]));
    const [date, setDate] = useState(props.item_info[3]);
    const [type, setType] = useState(props.item_info[4]);
    const id = props.item_info[0];

    useEffect(() => {
        setName(props.item_info[1]);
        setAmount(props.item_info[2]);
        setDate(props.item_info[3]);
        setType(props.item_info[4]);
    },[props.item_info])

    function edit_product(event) {
        event.preventDefault();
        event.stopPropagation();
        if(name === '') {
            showError('Name of a product cannot be empty.');
            return;
        }
        if(parseInt(amount) <= 0) {
            showError('The amount of products cannot be null or under it.');
            return;
        }
        if(new Date(date).getTime() < new Date().getTime()) {
            showError('A expiration date of a product cannot be lower than today');
            return;
        }
        props.edit_product({
            id : id,
            name : name,
            amount : parseInt(amount),
            date : date,
            kind : type
        });
    }
    
    return <Popup modal trigger={<button className="btn btn-primary modal-input">
            <PenIcon/>
        </button>}>
            {
                close => <div className="modal-dialog">
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
                    <button className="btn btn-primary modal-input" onClick={(e) => {
                        edit_product(e);
                        close();
                    }}>
                        Edit product
                    </button>
                </div>
            </div>
            }
        </Popup>;
}