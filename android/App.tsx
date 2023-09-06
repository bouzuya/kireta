import React from "react";
import { PaperProvider } from "react-native-paper";
import { NavigationContainer } from "@react-navigation/native";
import { createNativeStackNavigator } from "@react-navigation/native-stack";
import { HistoryScreen } from "./components/HistoryScreen";
import { ItemScreen } from "./components/ItemScreen";
import { ListScreen } from "./components/ListScreen";
import { TodayScreen } from "./components/TodayScreen";

const Stack = createNativeStackNavigator();

export default function App(): JSX.Element {
  return (
    <PaperProvider>
      <NavigationContainer>
        <Stack.Navigator initialRouteName="TodayScreen">
          <Stack.Screen name="HistoryScreen" component={HistoryScreen} />
          <Stack.Screen name="ItemScreen" component={ItemScreen} />
          <Stack.Screen name="ListScreen" component={ListScreen} />
          <Stack.Screen name="TodayScreen" component={TodayScreen} />
        </Stack.Navigator>
      </NavigationContainer>
    </PaperProvider>
  );
}
