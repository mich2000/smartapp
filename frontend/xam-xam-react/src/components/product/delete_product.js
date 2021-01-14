import React from 'react';
import Popup from 'reactjs-popup';

export function DeleteProductPopup(props) {
    function trigger_delete(event) { 
        let id = event.target.value;
        event.preventDefault();
        event.stopPropagation();
        props.delete_product(id);
    }

    return <Popup trigger={<button className="badge badge-pill badge-danger m-2" value={props.item[0]} type="button">
            <svg width="1em" height="1em" viewBox="0 0 16 16" className="bi bi-x" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
                <path fillRule="evenodd" d="M4.646 4.646a.5.5 0 0 1 .708 0L8 7.293l2.646-2.647a.5.5 0 0 1 .708.708L8.707 8l2.647 2.646a.5.5 0 0 1-.708.708L8 8.707l-2.646 2.647a.5.5 0 0 1-.708-.708L7.293 8 4.646 5.354a.5.5 0 0 1 0-.708z"/>
            </svg>
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