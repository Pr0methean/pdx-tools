import { Tooltip } from "antd";
import React, { useState } from "react";
import {
  SideBarButtonProps,
  SideBarButton,
} from "../../components/SideBarButton";
import { SideBarContainerProvider } from "../../components/SideBarContainer";
import { MapSettingsDrawer } from "./MapSettingsDrawer";

export const MapSettingsSideBarButton = ({
  children,
  ...props
}: SideBarButtonProps) => {
  const [drawerVisible, setDrawerVisible] = useState(false);

  return (
    <>
      <SideBarContainerProvider>
        <MapSettingsDrawer
          open={drawerVisible}
          closeDrawer={() => setDrawerVisible(false)}
        />
      </SideBarContainerProvider>
      <Tooltip title="Map and timelapse settings" placement="left">
        <SideBarButton {...props} onClick={() => setDrawerVisible(true)}>
          {children}
        </SideBarButton>
      </Tooltip>
    </>
  );
};
