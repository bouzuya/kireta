import { MaterialIcons } from "@expo/vector-icons";
import { createBottomTabNavigator } from "@react-navigation/bottom-tabs";
import { TabActions } from "@react-navigation/native";
import type { NativeStackScreenProps } from "@react-navigation/native-stack";
import React from "react";
import { BottomNavigation } from "react-native-paper";
import type {
  BottomTabParamList,
  NativeStackParamList,
} from "../types/navigation";
import { HistoryScreen } from "./HistoryScreen";
import { TodayScreen } from "./TodayScreen";

const Tab = createBottomTabNavigator<BottomTabParamList>();

type Props = NativeStackScreenProps<NativeStackParamList, "Tab">;

export function TabScreen(_: Props): JSX.Element {
  return (
    <Tab.Navigator
      tabBar={({ navigation, state, descriptors, insets }) => (
        <BottomNavigation.Bar
          getLabelText={({ route }): string | undefined => {
            const { tabBarLabel } = descriptors[route.key].options;
            return typeof tabBarLabel === "string" ? tabBarLabel : undefined;
          }}
          navigationState={state}
          onTabPress={({ route, preventDefault }) => {
            const event = navigation.emit({
              type: "tabPress",
              target: route.key,
              canPreventDefault: true,
            });

            if (event.defaultPrevented) {
              preventDefault();
            } else {
              navigation.dispatch(TabActions.jumpTo(route.name));
            }
          }}
          renderIcon={({ color, focused, route }): React.ReactNode => {
            const { tabBarIcon } = descriptors[route.key].options;
            return tabBarIcon === undefined
              ? null
              : tabBarIcon({ focused, color, size: 24 });
          }}
          safeAreaInsets={insets}
        />
      )}
    >
      <Tab.Screen
        component={TodayScreen}
        name="Today"
        options={{
          tabBarIcon: ({ color, size }) => (
            <MaterialIcons name="list-alt" size={size} color={color} />
          ),
          tabBarLabel: "Today",
        }}
      />
      <Tab.Screen
        component={HistoryScreen}
        name="History"
        options={{
          tabBarIcon: ({ color, size }) => (
            <MaterialIcons name="history" size={size} color={color} />
          ),
          tabBarLabel: "History",
        }}
      />
    </Tab.Navigator>
  );
}
