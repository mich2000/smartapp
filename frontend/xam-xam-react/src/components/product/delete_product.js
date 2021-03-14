import React from 'react';
import Popup from 'reactjs-popup';
import {DeleteIcon} from '../../icon';

export function DeleteProductPopup(props) {
    function trigger_delete(event) { 
        let id = event.target.value;
        event.preventDefault();
        event.stopPropagation();
        props.delete_product(id);
    }

    return <Popup trigger={<button className="badge badge-pill badge-danger m-2" value={props.item[0]} type="button">
            <DeleteIcon/>
        </button>}
        modal nested>
            {
                close => <div className="modal-dialog">
                    <div className="modal-header">Are you sure to remove the product?</div>
                    <div className="modal-content">
                        <button className="btn btn-primary modal-input" value={props.item[0]}
                        onClick={(e) => {
                            trigger_delete(e);
                            close();
                        }}>
                            Remove {props.item[1]}
                        </button>
                    </div>
                </div>
            }
        </Popup>;
}