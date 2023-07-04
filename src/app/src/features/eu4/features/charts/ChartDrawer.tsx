import React, { useState } from "react";
import { Drawer } from "antd";
import { VizRenderer } from "./VizRenderer";
import { DisplayLimitAlert } from "./DisplayLimitAlert";
import { VizModules } from "../../types/visualizations";
import { Help } from "./Help";
import {
  closeDrawerPropagation,
  useSideBarContainerRef,
} from "../../components/SideBarContainer";
import { ChartDrawerTitle } from "./ChartDrawerTitle";

interface ChartDrawerProps {
  open: boolean;
  closeDrawer: () => void;
}

const vizModuleDisplayLimit = (module: VizModules) => {
  switch (module) {
    case "owned-development-states":
      return 12;
    case "monthly-income":
    case "score":
    case "nation-size":
    case "inflation":
    case "health":
      return 30;
    default:
      return null;
  }
};

const DEFAULT_VIZUALIZATION_SELECTION = "monthly-income";

export const ChartDrawer = ({ open, closeDrawer }: ChartDrawerProps) => {
  const [selectedViz, setSelectedViz] = useState<VizModules>(
    DEFAULT_VIZUALIZATION_SELECTION
  );
  const [expanded, setExpanded] = useState(false);
  const [helpVisible, setHelpVisible] = useState(false);
  const displayLimit = vizModuleDisplayLimit(selectedViz);
  const sideBarContainerRef = useSideBarContainerRef();

  return (
    <Drawer
      open={open}
      closable={true}
      mask={false}
      maskClosable={false}
      push={!helpVisible}
      onClose={closeDrawerPropagation(closeDrawer, open)}
      width={!expanded ? "min(800px, 100%)" : "100%"}
      title={
        <ChartDrawerTitle
          selectedViz={selectedViz}
          setSelectedViz={setSelectedViz}
          expanded={expanded}
          setExpanded={setExpanded}
          onHelp={() => setHelpVisible(true)}
        />
      }
    >
      <Drawer open={helpVisible} onClose={() => setHelpVisible(false)}>
        <Help module={selectedViz} />
      </Drawer>

      <div className="flex h-full flex-col gap-2" ref={sideBarContainerRef}>
        {displayLimit && <DisplayLimitAlert displayLimit={displayLimit} />}
        <VizRenderer module={selectedViz} />
      </div>
    </Drawer>
  );
};
