import {FETCH_TASKS, SET_LOADING,API_ERROR} from './types';
import TodoApi from "../components/gateway/todoApi";

// fetch tasks from server
export const fetchTasks = () => async (dispatch:any) => {
  try {
    setLoading();

    const res = await TodoApi.GET('/tasks');
    // const data = await res.json();

    dispatch({
      type: FETCH_TASKS,
      payload: res.tasks
    });
  } catch (err) {
    dispatch({
      type: API_ERROR,
      payload: err.response.data
    })
  }
};

export const setLoading = () => {
  return {
    type: SET_LOADING
  }
}
