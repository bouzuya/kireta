import { NavigationContainer } from "@react-navigation/native";
import { createNativeStackNavigator } from "@react-navigation/native-stack";
import React from "react";
import { PaperProvider } from "react-native-paper";
import { ItemScreen } from "./components/ItemScreen";
import { ListScreen } from "./components/ListScreen";
import { StoreContextProvider } from "./components/StoreContext";
import { TabScreen } from "./components/TabScreen";
import type { NativeStackParamList } from "./types/navigation";

const Stack = createNativeStackNavigator<NativeStackParamList>();

// eslint-disable-next-line import/no-default-export
export default function App(): JSX.Element {
  return (
    <PaperProvider>
      <StoreContextProvider>
        <NavigationContainer>
          <Stack.Navigator initialRouteName="Tab">
            <Stack.Screen name="Item" component={ItemScreen} />
            <Stack.Screen name="List" component={ListScreen} />
            <Stack.Screen
              name="Tab"
              component={TabScreen}
              options={{ headerShown: false }}
            />
          </Stack.Navigator>
        </NavigationContainer>
      </StoreContextProvider>
    </PaperProvider>
  );
}
