from trust.table import column_stats_category
from trust.table import column_profile_abstraction
from trust.table import column_chart_category



class ColumnProfileCategoryDto(column_profile_abstraction.ColumnProfileAbstractionDto):

    frequent_values: column_chart_category.ColumnChartCategoryDto
    stats: column_stats_category.ColumnStatsCategoryDto
