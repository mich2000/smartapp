import api_functions from '../../api';
import React, { useState,useEffect }  from 'react';
import {ToastContainer} from 'react-toastify';
import {showError} from '../../toast';

export function UserInfo() {
    const [amount_prod,SetAmountProd] = useState(0);
    const [amount_stor,SetAmountStor] = useState(0);
    const [min_date,SetMinDate] = useState(null);
    const [max_date,SetMaxDate] = useState(null);

    useEffect(() => {
        fetch(api_functions.get_api() + "/user/basic/info",api_functions.method_get())
        .then((api_call) => {
            if(api_call.status === 200) {
                api_call.json()
                .then(json => {
                    SetAmountProd(json.amount_product);
                    SetAmountStor(json.amount_storage);
                    SetMinDate(json.min_bederf);
                    SetMaxDate(json.max_bederf);
                })
                .catch((e) => showError(`Could not send through the request. error: ${e}`));
            } else {
                api_call.text()
                .then(text => console.log(text))
                .catch((e) => showError(`Json related error: ${e}`));
            }
        })
        .catch((e) => showError(`Could not send through the request. error: ${e}`));
    },[])

    return (
        <div className="user-info">
            <div>
                <div>
                    <dt># Storages</dt>
                    <dd>{amount_stor}</dd>
                </div>
                <div>
                    <dt># products</dt>
                    <dd>{amount_prod}</dd>
                </div>
            </div>
            { (amount_prod !== 0) &&
                <div>
                    <div>
                        <dt>Max. expiration date</dt>
                        <dd>{max_date}</dd>
                    </div>
                    <div>
                        <dt>Min. expiration date</dt>
                        <dd>{min_date}</dd>
                    </div>
                </div>
            }
            <ToastContainer/>
        </div>
    );
}