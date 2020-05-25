import {FETCH_TASKS, SET_LOADING, API_ERROR, ADD_TASK} from '../actions/types'

const initialState = {
  tasks: null,
  current: null,
  loading: false,
  error: null,
}

export default (state:any = initialState, action:any) => {
 switch(action.type) {
   case FETCH_TASKS:
     return {
       ...state,
       tasks: action.payload
     }
   case ADD_TASK:
     return {
       ...state,
       tasks: [...state.tasks, action.payload],
     };
   case SET_LOADING:
     return {
       ...state,
       loading: true,
     };
   case API_ERROR:
     console.error(action.payload);
     return {
       ...state,
       error: action.payload
     }
   default:
     return state;
 }
}
