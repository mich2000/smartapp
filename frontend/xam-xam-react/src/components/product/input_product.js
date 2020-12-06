import React, { useState }  from 'react';
import Popup from 'reactjs-popup';
import api_functions from '../../api';
import {ProductType} from '../../enums';

export const InputProductDialog = (props) => {
    const [name, setName] = useState('');
    const [amount, setAmount] = useState(0);
    const [date, setDate] = useState(new Date());
    const [type, setType] = useState(ProductType.Other);
    const storage = props.storage;

    function add_product(event) {
        event.preventDefault();
        event.stopPropagation();
        let options = api_functions.method_post();
        options.body = JSON.stringify({
            storage_name: storage,
            name: name,
            amount: parseInt(amount),
            peremption_date: date,
            kind: type
        });
        fetch(api_functions.get_business_api() + '/product', options)
        .then((api_call) => {
            if(api_call.status === 200) {
                api_call.json()
                .then(json => {
                    props.add_product([json.product_id, name, amount,date,type]);
                });
                setType(ProductType.Other);
                setDate(new Date());
                setName('');
                setAmount(0);
            }
        }).catch((e) => {
            console.error(`Could not send through the request. error: ${e}`);
        });
    }
    
    return (
        <Popup trigger={<button className="btn btn-primary modal-input">Add product</button>} modal nested>
            {
                close => (
                    <div className="modal-dialog">
                        <div className="modal-header">Add product</div>
                        <div className="modal-content">
                            <input className="modal-input form-control" type="text" value={name} placeholder="Enter the product name" onChange={(e) => setName(e.target.value)}/>
                            <input className="modal-input form-control" type="number" value={amount} placeholder="Enter the amount" onChange={(e) => setAmount(e.target.value)}/>
                            <input className="modal-input form-control" type="date" value={date} placeholder="Enter the expiration date" onChange={(e) => setDate(e.target.value)}/>
                            <select className="modal-input form-control" value={type} onChange={(e) => setType(e.target.value)}>
                                {Object.keys(ProductType).map(key => (
                                    <option key={key} value={key}>
                                        {key}
                                    </option>
                                    )
                                )}
                            </select>
                            <button className="btn btn-primary modal-input" onClick={(e) => {add_product(e);close();}}>
                                Add product
                            </button>
                        </div>
                    </div>
                )
            }
        </Popup>
    );
}