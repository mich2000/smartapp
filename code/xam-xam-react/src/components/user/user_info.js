import api_functions from '../../api';
import React, { useState,useEffect,useContext }  from 'react';
import {AppContext} from '../../state';

export function UserInfo() {
    const [amount_prod,SetAmountProd] = useState(0);
    const [amount_stor,SetAmountStor] = useState(0);
    const [min_date,SetMinDate] = useState(null);
    const [max_date,SetMaxDate] = useState(null);
    const [user,setUser] = useContext(AppContext);

    useEffect(() => {
        fetch(api_functions.get_api() + "/user/basic/info",api_functions.method_get())
        .then((api_call) => {
            if(api_call.status === 200) {
                api_call.json()
                .then(json => {
                    SetAmountProd(json.amount_storage);
                    SetAmountStor(json.amount_product);
                    SetMinDate(json.min_bederf);
                    SetMaxDate(json.max_bederf);
                    if(json.email !== user.email) { 
                        setUser({email : json.email, loggedIn : user.loggedIn});
                    }
                })
                .catch((e) => console.error(`Could not send through the request. error: ${e}`));
            } else {
                console.log(api_call.body)
            }
        },[setUser])
        .catch((e) => console.error(`Could not send through the request. error: ${e}`));
    })

    return (
        <div>
            <div>
                <dt>Amount of Storages</dt>
                <dd>{amount_stor}</dd>
            </div>
            <div>
                <dt>Amount of products</dt>
                <dd>{amount_prod}</dd>
            </div>
            { (amount_prod !== 0) && 
                <div>
                    <div>
                        <dt>Maximal peremption date</dt>
                        <dd>{max_date}</dd>
                    </div>
                    <div>
                        <dt>Minimal peremption date</dt>
                        <dd>{min_date}</dd>
                    </div>
                </div>
            }
        </div>
    );
}