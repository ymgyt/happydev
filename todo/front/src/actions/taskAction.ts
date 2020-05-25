import {
  FETCH_TASKS,
  SET_LOADING,
  API_ERROR,
  ADD_TASK,
  UNSET_LOADING,
  CLOSE_ADD_TASK_MODAL,
  OPEN_ADD_TASK_MODAL
} from './types';
import TodoApi from "../gateway/todoApi";

// fetch tasks from server
export const fetchTasks = () => async (dispatch: any) => {
  try {
    // このあたりは前後の処理を抽象化してclosureわたすような感じで書きたい
    // withLoading(...)みたいな
    await setLoading(dispatch);
    const data = await TodoApi.getTasks({});
    await unsetLoading(dispatch);

    dispatch({
      type: FETCH_TASKS,
      payload: data.tasks
    });
  } catch (err) {
    dispatch({
      type: API_ERROR,
      payload: err.response
    })
  }
};

// add new task to server
export const addTask = (task: any) => async (dispatch: any) => {
  try {
    await setLoading(dispatch);
    const data = await TodoApi.createTask(task);
    await unsetLoading(dispatch);

    dispatch({
      type: ADD_TASK,
      payload: data,
    })
  } catch (err) {
    dispatch({
      type: API_ERROR,
      payload: err.response
    })
  }
}

export const setLoading = async (dispatch: any) => dispatch({type: SET_LOADING})
export const unsetLoading = async (dispatch: any) => dispatch({type: UNSET_LOADING})
export const openAddTaskModal = () => async (dispatch: any) => dispatch({type: OPEN_ADD_TASK_MODAL})
export const closeAddTaskModal = () => async (dispatch: any) => dispatch({type: CLOSE_ADD_TASK_MODAL})
