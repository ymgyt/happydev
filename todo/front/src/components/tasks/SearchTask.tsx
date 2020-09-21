import React, {KeyboardEvent} from 'react';
import {connect} from 'react-redux';
import InputBase from '@material-ui/core/InputBase';
import { makeStyles } from '@material-ui/core/styles';
import { fetchTasks } from '../../actions/taskAction';

const useStyles = makeStyles((theme) => ({
  inputRoot: {
    color: 'inherit',
    width: '100%',
  },
  inputInput: {
    padding: '8px 8px 8px 0',
    // vertical padding + font size from searchIcon
    paddingLeft: `calc(1em + ${theme.spacing(4)}px)`,
    transition: theme.transitions.create('width'),
  },
}));

interface SearchTaskProps {
  fetchTasks: any,
}

const SearchTask = (props: SearchTaskProps) => {
  const {fetchTasks} = props;
  const classes = useStyles();

  const [query, setQuery] = React.useState<string | undefined>("");
  // 同じqueryで連続してfetchが走らないようにしたい
  const [repeated, setRepeated] = React.useState<boolean | undefined>(false);

  const handleChange = (event: any) => {
    setRepeated(false);
    setQuery(event.target.value)
  }

  // Enterを押したらfetchを実行する
  const handleKeyPress = (event: KeyboardEvent) => {
    if ((event.which === 13 || event.keyCode === 13) && !repeated) {
      setRepeated(true); // 一度fetchを実行したらkeyが押されてqueryがかわるまでは実行しない
      fetchTasks({query})
    }
  }

  return (<InputBase
      placeholder='Search Tasks'
      onChange={handleChange}
      inputProps={{onKeyPress: handleKeyPress}}
      classes={{
        root: classes.inputRoot,
        input: classes.inputInput,
      }}
    />
  )
}

export default connect(
  null,
  {fetchTasks}
)(SearchTask)


