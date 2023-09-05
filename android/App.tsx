import { StatusBar } from "expo-status-bar";
import { useState } from "react";
import { FlatList, StyleSheet, View } from "react-native";
import { Checkbox, FAB, List, PaperProvider } from "react-native-paper";
import { SafeAreaView } from "react-native-safe-area-context";

export default function App(): JSX.Element {
  const [items, setItems] = useState<{ id: number; name: string }[]>([]);
  const [checked, setChecked] = useState<Record<number, boolean>>({});
  return (
    <PaperProvider>
      <SafeAreaView>
        <View style={styles.container}>
          <StatusBar style="auto" />
          <FlatList
            data={items}
            keyExtractor={(item) => item.id.toString()}
            renderItem={({ item }) => (
              <List.Item
                left={(props) => (
                  <Checkbox
                    onPress={() =>
                      setChecked((prev) => ({
                        ...prev,
                        [item.id]: !prev[item.id],
                      }))
                    }
                    status={checked[item.id] ? "checked" : "unchecked"}
                    {...props}
                  />
                )}
                title={item.name}
              />
            )}
          />
        </View>
        <FAB
          icon="plus"
          onPress={() => {
            setItems((prev) => {
              const id = (prev[prev.length - 1]?.id ?? 0) + 1;
              return [...prev, { id, name: `Item ${id}` }];
            });
          }}
          style={{ position: "absolute", right: 16, bottom: 16 }}
        />
      </SafeAreaView>
    </PaperProvider>
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
