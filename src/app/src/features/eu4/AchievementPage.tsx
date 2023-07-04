import React, { useMemo } from "react";
import { Typography, Spin } from "antd";
import { RecordTable } from "./components/RecordTable";
import { Achievement, useAchievementQuery } from "@/services/appApi";
const { Paragraph } = Typography;

interface AchievementRoute {
  achievementId: string;
  staticAchievement?: Achievement;
}

const useAchievement = (achievementId: string) => {
  const achievementQuery = useAchievementQuery(achievementId);
  const achievement = achievementQuery.data?.achievement;
  const saves = useMemo(
    () =>
      (achievementQuery.data?.saves ?? []).map((x, i) => ({
        ...x,
        rank: i + 1,
      })),
    [achievementQuery.data]
  );

  return { isFetching: achievementQuery.isFetching, achievement, saves };
};

export const AchievementPage = ({
  achievementId,
  staticAchievement,
}: AchievementRoute) => {
  const { achievement, saves } = useAchievement(achievementId);

  const title = achievement?.name ?? staticAchievement?.name ?? "";
  const table = saves === undefined ? null : <RecordTable records={saves} />;
  const description =
    achievement?.description ?? staticAchievement?.description ?? "";

  return (
    <div className="p-5 mx-auto max-w-7xl">
      <h1 className="text-4xl">{title}</h1>
      <p>Achievement id: {achievementId}</p>
      <Paragraph>{description}</Paragraph>
      {table}
    </div>
  );
};
