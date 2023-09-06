import { StackActions, useNavigation } from "@react-navigation/native";
import React from "react";
import { StyleSheet, View } from "react-native";
import { Button } from "react-native-paper";

export function ListScreen(): JSX.Element {
  const navigation = useNavigation();
  return (
    <View style={styles.container}>
      <Button
        onPress={() => { navigation.dispatch(StackActions.push("ItemScreen")); }}
      >
        Item 1
      </Button>
      <Button
        onPress={() => { navigation.dispatch(StackActions.push("ItemScreen")); }}
      >
        Item 2
      </Button>
    </View>
  );
}

const styles = StyleSheet.create({
  container: {
    alignContent: "flex-start",
    alignItems: "stretch",
    display: "flex",
    flexDirection: "column",
    height: "100%",
    justifyContent: "flex-start",
    margin: 0,
    padding: 0,
    width: "100%",
  },
});
