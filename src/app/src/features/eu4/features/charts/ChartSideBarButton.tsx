import React, { useState } from "react";
import {
  SideBarButton,
  SideBarButtonProps,
} from "../../components/SideBarButton";
import { VisualizationProvider } from "@/components/viz";
import { ChartDrawer } from "./ChartDrawer";
import { SideBarContainerProvider } from "../../components/SideBarContainer";
import { Tooltip } from "antd";

export const ChartSideBarButton = ({
  children,
  ...props
}: SideBarButtonProps) => {
  const [drawerVisible, setDrawerVisible] = useState(false);

  return (
    <>
      <VisualizationProvider>
        <SideBarContainerProvider>
          <ChartDrawer
            open={drawerVisible}
            closeDrawer={() => setDrawerVisible(false)}
          />
        </SideBarContainerProvider>
      </VisualizationProvider>
      <Tooltip title="Worldwide charts and tables" placement="left">
        <SideBarButton {...props} onClick={() => setDrawerVisible(true)}>
          {children}
        </SideBarButton>
      </Tooltip>
    </>
  );
};
