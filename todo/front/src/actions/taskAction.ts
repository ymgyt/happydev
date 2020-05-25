import {FETCH_TASKS, SET_LOADING,API_ERROR,ADD_TASK} from './types';
import TodoApi from "../gateway/todoApi";

// fetch tasks from server
export const fetchTasks = () => async (dispatch:any) => {
  try {
    setLoading();

    const data = await TodoApi.GET('/tasks');

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

// add new task
export const addTask = (task:any) => async (dispatch:any) => {
  try{
    setLoading();
    const data = await TodoApi.POST('/tasks', task);

    dispatch({
      type: ADD_TASK,
      payload: data.task,
    })

  } catch(err) {
    dispatch({
      type: API_ERROR,
      payload: err.response
    })
  }
}

export const setLoading = () => {
  return {
    type: SET_LOADING
  }
}
