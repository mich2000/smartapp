import React, { useState }  from 'react';
import Popup from 'reactjs-popup';
import api_functions from '../../api';
import {ProductType} from '../../enums';
import {showError} from '../../toast';

export function nowDatePlusDays(days,event) {
    event.preventDefault();
    event.stopPropagation();
    const newDate = new Date();
    newDate.setDate(newDate.getDate() + days);
    let month = newDate.getMonth() > 9 ? newDate.getMonth() : '0' + newDate.getMonth();
    let day = newDate.getDate() > 9 ? newDate.getDate() : '0' + newDate.getDate();
    return `${newDate.getFullYear()}-${month}-${day}`;
}

export function nowDatePlusMonth(months, event) {
    event.preventDefault();
    event.stopPropagation();
    const newDate = new Date();
    newDate.setMonth(newDate.getMonth() + months);
    let month = newDate.getMonth() > 9 ? newDate.getMonth() : '0' + newDate.getMonth();
    let day = newDate.getDate() > 9 ? newDate.getDate() : '0' + newDate.getDate();
    return `${newDate.getFullYear()}-${month}-${day}`;
}

export const InputProductDialog = (props) => {
    const [name, setName] = useState('');
    const [amount, setAmount] = useState(1);
    const [date, setDate] = useState(new Date());
    const [type, setType] = useState(ProductType.Other);
    const storage = props.storage;

    function add_product(event) {
        event.preventDefault();
        event.stopPropagation();
        let options = api_functions.method_post();
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
            } else {
                api_call.text()
                .then(err => showError(err));
            }
        }).catch(() => showError('No internet connection'));
    }

    return <Popup trigger={<button className="btn btn-primary modal-input">
            <svg width="1em" height="1em" viewBox="0 0 16 16" className="bi bi-plus svg-plus" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
                <path fillRule="evenodd" d="M8 4a.5.5 0 0 1 .5.5v3h3a.5.5 0 0 1 0 1h-3v3a.5.5 0 0 1-1 0v-3h-3a.5.5 0 0 1 0-1h3v-3A.5.5 0 0 1 8 4z"/>
            </svg>
        </button>} modal nested>
            {
                close => <div className="modal-dialog">
                <div className="modal-header">Add product</div>
                <div className="modal-content">
                    <input className="modal-input form-control" type="text" value={name} placeholder="Enter the product name" onChange={(e) => setName(e.target.value)}/>
                    <input className="modal-input form-control" min="1" type="number" value={amount} placeholder="Enter the amount" onChange={(e) => setAmount(e.target.value)}/>
                    <label>Expires in</label>
                    <div className="user-info">
                        <input className="modal-input form-control" type="button" value="3 days" onClick={(e) => setDate(nowDatePlusDays(3,e))}/>
                        <input className="modal-input form-control" type="button" value="7 days" onClick={(e) => setDate(nowDatePlusDays(7,e))}/>
                        <input className="modal-input form-control" type="button" value="14 days" onClick={(e) => setDate(nowDatePlusDays(14,e))}/>
                    </div>
                    <div className="user-info">
                        <input className="modal-input form-control" type="button" value="1 month" onClick={(e) => setDate(nowDatePlusMonth(1,e))}/>
                        <input className="modal-input form-control" type="button" value="2 months" onClick={(e) => setDate(nowDatePlusMonth(2,e))}/>
                    </div>
                    <input className="modal-input form-control" type="date" value={date} placeholder="Enter the expiration date" onChange={(e) => setDate(e.target.value)}/>
                    <select className="modal-input form-control" value={type} placeholder={date} onChange={(e) => setType(e.target.value)}>
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
            }
        </Popup>;
}