SELECT
    id,
    name,
    estimate,
    math::sum((SELECT VALUE amount FROM income WHERE id[1] = $parent.id)) as income,
    math::sum((SELECT VALUE amount FROM expense WHERE id[1] = $parent.id)) as expense
FROM type::thing("project", $record);