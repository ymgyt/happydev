import {
  FETCH_TASKS,
  SET_LOADING,
  API_ERROR,
  ADD_TASK,
  UNSET_LOADING,
  OPEN_ADD_TASK_MODAL,
  CLOSE_ADD_TASK_MODAL
} from '../actions/types'

const initialState = {
  tasks: null,
  current: null,
  loading: false,
  openAddTaskModal: false,
  error: null,
}

export default (state: any = initialState, action: any) => {
  switch (action.type) {
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
    case UNSET_LOADING:
      return {
        ...state,
        loading: false,
      };
    case OPEN_ADD_TASK_MODAL:
      return {
        ...state,
        openAddTaskModal: true,
      };
    case CLOSE_ADD_TASK_MODAL:
      return {
        ...state,
        openAddTaskModal: false,
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
