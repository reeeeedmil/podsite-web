# Generated by Django 5.0 on 2024-02-19 09:25

from django.db import migrations, models


class Migration(migrations.Migration):

    dependencies = [
        ('subNet', '0007_alter_net_title_alter_net_title_slug'),
    ]

    operations = [
        migrations.AlterField(
            model_name='net',
            name='title_slug',
            field=models.SlugField(blank=True, max_length=150, null=True),
        ),
    ]