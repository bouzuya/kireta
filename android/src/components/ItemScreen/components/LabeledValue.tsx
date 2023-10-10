import React from "react";
import { StyleSheet, Text, View } from "react-native";

type Props = {
  label: string;
  value: string;
};

export function LabeledValue({ label, value }: Props): JSX.Element {
  return (
    <View style={styles.container}>
      <Text>{label}</Text>
      <Text style={styles.valueText}>{value}</Text>
    </View>
  );
}

const styles = StyleSheet.create({
  container: {
    paddingHorizontal: 16,
  },
  valueText: {
    borderBottomColor: "transparent",
    borderBottomWidth: 2,
    fontSize: 16,
    padding: 16,
  },
});
